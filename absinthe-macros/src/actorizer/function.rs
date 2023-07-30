use crate::dev::prelude::*;

pub struct ActorizeFn(ItemFn);

impl Parse for ActorizeFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let actorizer = Self(input.parse::<ItemFn>()?);
  
        let is_method = actorizer.0.sig.receiver().is_some();
        let is_async = actorizer.0.sig.asyncness.is_some();

        if ! is_async {
            return Err(syn::Error::new_spanned(actorizer.0.sig.fn_token, "The `actor_fn` attribute can only be used on async functions!"))
        }
        else if is_method {
            return Err(syn::Error::new_spanned(actorizer.0.sig.fn_token, "The `actor_fn` attribute cannot be used on associated functions or methods!"))
        }

        Ok(actorizer)
    }
}

impl Into<TokenStream> for ActorizeFn  {
    fn into(self) -> TokenStream {
        self.into_token_stream().into()
    }
}

impl ToTokens for ActorizeFn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let fn_impl = &self.0;

        let fn_name = &fn_impl.sig.ident;
        let struct_name = format_ident!("{}ActorFn", heck::AsUpperCamelCase(fn_name.to_string()).to_string());

        let fn_args = &fn_impl.sig.inputs;
        // Unpack message tuple to function arguments
        let fn_args_unpack = match fn_args.len() {
            0 => quote!(),
            1 => quote!(req),
            _ => {
                let unpack: Vec<_> = fn_args.iter().enumerate().map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote!(req.#i)
                    })
                    .collect();
                quote!(#(#unpack),*)
            },
        };
        let fn_args_t: Vec<_> = fn_args.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                Some(&pat_type.ty)
            } else {
                None
            }
        }).collect();
        // Request type is 
        // Unit if there are no arguments
        // A single type if there is one argument
        // A tuple of types if there are multiple arguments
        let fn_args_t = match fn_args.len() {
            0 => quote!(()),
            1 => quote!(#(#fn_args_t),*),
            _ => quote!((#(#fn_args_t),*)),
        };

        let fn_ret = match &fn_impl.sig.output {
            syn::ReturnType::Default => quote!(()),
            syn::ReturnType::Type(_, type_boxed) => quote!(#type_boxed),   
        };

        let fn_block = &fn_impl.block;

        let is_generic = ! fn_impl.sig.generics.params.is_empty();

        let (generics, generic_params, where_clause) = if is_generic {
            let generics = &fn_impl.sig.generics;

            let generic_params: Vec<_> = fn_impl.sig.generics.params.iter().map(|param| {
                match param {
                    GenericParam::Type(type_param) => {
                        let type_param_ident = &type_param.ident;
                        quote!(#type_param_ident)
                    },
                    GenericParam::Lifetime(lifetime_def) => {
                        let lifetime_def = &lifetime_def.lifetime;
                        quote!(#lifetime_def)
                    },
                    GenericParam::Const(const_param) => {
                        let const_param_ident = &const_param.ident;
                        quote!(#const_param_ident)
                    },
                }
            }).collect();

            let where_clause = &fn_impl.sig.generics.where_clause;
            
            let mut where_predicates: Vec<_> = fn_impl.sig.generics.params.iter().map(|param| {
                match param {
                    GenericParam::Type(type_param) => {
                        let type_param_ident = &type_param.ident;
                        quote!(#type_param_ident: Send + Sync + 'static,)
                    },
                    _ => quote!()
                }
            }).collect();
            
            if let Some(where_clause) = where_clause {
                where_predicates.extend(where_clause.predicates.iter().map(|predicate| {
                    quote!(#predicate,)
                }));
            }
            
            (Some(generics), Some(quote!(<#(#generic_params),*> )), Some(quote!(where #(#where_predicates)*)))
        } else {
            (None, None, None)
        };

        let struct_def = if is_generic {
            quote!(
                {
                    _marker: std::marker::PhantomData #generic_params,
                }
            )
        } else {
            quote!(;)
        };

        let struct_init = if is_generic {
            quote!(
                Self {
                    _marker: std::marker::PhantomData,
                }
            )
        } else {
            quote!(
                Self
            )
        };


        let output = quote! {
            struct #struct_name #generics 
            #where_clause 
            #struct_def

            impl #generics #struct_name #generic_params
            #where_clause
            {
                fn new() -> Self {
                    #struct_init
                }

                async fn #fn_name(#fn_args) -> #fn_ret
                #fn_block
            }

            #[absinthe::prelude::async_trait]
            impl #generics Actor for #struct_name #generic_params  
            #where_clause
            {
                type Request  = #fn_args_t;
                type Response = #fn_ret;

                async fn recv_msg(&self, req: Self::Request) -> Self::Response {
                    Self::#fn_name(#fn_args_unpack).await
                }
            }

            fn #fn_name #generics () -> absinthe::actor::ActorHandle<#struct_name #generic_params> #where_clause {
                let actor = #struct_name::new();
                absinthe::spawn(actor)
            }
        };

        output.to_tokens(tokens);
    }
}