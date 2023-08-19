use crate::dev::prelude::*;

#[cfg(test)]
mod tests;

mod model;
mod parser;

pub use model::*;

pub struct ActorCodeGen;

impl ICodeGen for ActorCodeGen
{
    type AttrModel = ActorAttrModel;
    type Model = ActorModel;

    fn codegen(attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream {
        let attr = attr.unwrap();

        let vis = &model.vis;
        let (impl_generics, ty_generics, where_clause) = model.generics.split_for_impl();
        let name = match attr.name {
            Some(name) => name,
            None => format_ident!("{}Actor", heck::AsUpperCamelCase(&model.name.to_string()).to_string()),
        };

        let where_clause = if ! model.generics.params.is_empty() {
            let mut where_clause = WhereClause {
                where_token: Token![where](proc_macro2::Span::call_site()),
                predicates: match where_clause {
                    Some(where_clause) => where_clause.predicates.clone(),
                    None => Punctuated::new(),
                },
            };
            
            let msg_predicates = ty_generics.clone()
                .into_token_stream()
                .to_string()
                .split(['<', '>', ' ', ','].as_ref())
                .filter(|s| ! s.is_empty())
                .fold(
                    Punctuated::<WherePredicate,Token![,]>::new(), 
                    |mut acc, s| {
                        let s = Ident::new(s, proc_macro2::Span::call_site());
                        acc.push(
                            parse_quote! {
                                #s: Send + Sync + 'static
                            }
                        );
                        acc
                    });


            where_clause.predicates.extend(msg_predicates);

            Some(where_clause)
        } else {
            None
        };

        let (phantom_gen, new_phantom_gen) = if ! model.generics.params.is_empty() {
            (
                quote! {
                    _phantom_gen: std::marker::PhantomData #ty_generics,
                },
                quote! {
                    _phantom_gen: std::marker::PhantomData,
                },
            )
        } else {
            (quote!{}, quote!{})
        };

        let (actor_state, new_actor_state) = if let Some(state_struct) = &model.state_struct {
            let state_fields = state_struct.fields.iter().map(|field| {
                let ident = &field.ident.clone().unwrap();
                let ty = &field.ty;

                quote! {
                    #ident: #ty,
                }
            });

            let state_fields_init = state_struct.fields.iter().map(|field| {
                let ident = &field.ident.clone().unwrap();
                let init_attr = field.attrs.iter().find_map(|attr| {
                    if attr.path().is_ident("init") {
                        Some(attr.parse_args::<Expr>().map_err(|e| e.to_compile_error()).unwrap())
                    } else {
                        None
                    }
                }).unwrap();

                quote! {
                    #ident: #init_attr
                }                
            });

            (
                quote! {
                    #(#state_fields)*
                },
                quote! {
                    #(#state_fields_init,)*
                },
            )
        } else {
            (quote!{}, quote!{})
        };

        let actor_fn = {
            let mut actor_fn = model.actor_fn.clone();
            actor_fn.block.stmts.retain(|stmt| {
                if let Stmt::Item(Item::Struct(state_struct)) = stmt {
                    state_struct.ident != "State"
                } else {
                    true
                }
            });
            actor_fn.sig.generics = Generics::default();

            actor_fn
        };

        let receive_fn = {
            let handler_name = &actor_fn.sig.ident; 
            let unpack_msg = match &model.req_t {
                Type::Tuple(tuple) => {
                    let unpack_req = tuple.elems.iter().enumerate().map(|(i, _)| {
                        let i = Index::from(i);
                        quote! {
                            msg.#i
                        }
                    });

                    quote! {
                        #(#unpack_req),*
                    }
                },
                _ => quote!{
                    msg
                },
            };

            let receiver = if actor_fn.sig.receiver().is_some() {
                quote!(self,)
            } else {
                quote!()
            };

            quote! {
                async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                    Self::#handler_name(#receiver #unpack_msg).await
                }
            }
        };

        let start_fn = quote! {
            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self {
                    #new_actor_state
                    #new_phantom_gen
                })
            }
        };
        let stop_fn: Option<ItemFn> = None;

        let req_t = &model.req_t;
        let resp_t = &model.resp_t;

        quote! {
            #vis struct #name #impl_generics #where_clause 
            {
                #actor_state
                #phantom_gen
            }

            impl #impl_generics #name #ty_generics #where_clause 
            {   
                #actor_fn
            }

            #[absinthe::prelude::async_trait]
            impl #impl_generics absinthe::Actor for #name #ty_generics #where_clause 
            {
                type Request = #req_t;
                type Response = #resp_t;

                #receive_fn
                #start_fn
                #stop_fn
            }
        }
    }
}
