use crate::dev::prelude::*;


pub struct ActorizeStruct(ItemStruct, ItemImpl);

impl Parse for ActorizeStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut struct_item = None;
        let mut impl_item = None;
        loop {
            if input.is_empty() {
                break;
            }
            else if input.peek(Token![struct]) {
                struct_item = Some(input.parse::<ItemStruct>()?);
            }
            else if input.peek(Token![impl]) {
                impl_item = Some(input.parse::<ItemImpl>()?);
            }
            else {
                return Err(input.error("Expected `struct` or `impl`"));
            }
        }

        if struct_item.is_none() {
            return Err(input.error("Expected `struct`"));
        }
        else if impl_item.is_none() {
            return Err(input.error("Expected `impl`"));
        }

        let struct_item = struct_item.unwrap();
        let impl_item = impl_item.unwrap();

        let impl_item_ident = match &*impl_item.self_ty {
            syn::Type::Path(tp) => tp.path.get_ident().unwrap(),
            _ => return Err(input.error("Expected `impl` to be a path")),
        };

        if struct_item.ident != *impl_item_ident {
            return Err(input.error("Expected `struct` and `impl` to have the same name"));
        }

        Ok(Self(struct_item, impl_item))
    }
}

impl Into<TokenStream> for ActorizeStruct  {
    fn into(self) -> TokenStream {
        self.into_token_stream().into()
    }
}


impl ToTokens for ActorizeStruct {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (struct_item, impl_item) = (&self.0, &self.1);

        let vis = &struct_item.vis;

        let generics = &struct_item.generics;
        let generic_params = &struct_item.generics.params;
        let where_clause = &struct_item.generics.where_clause;

        let actor_name = &struct_item.ident;
        let actor_req_name = format_ident!("{}Req", actor_name);
        let actor_resp_name = format_ident!("{}Resp", actor_name);


        let msg_enum_values = impl_item.items.iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(fn_item) => Some(fn_item),
                _ => None,
            })
            .filter(|fn_item| fn_item.attrs.iter().any(|attr| attr.path().is_ident("act")))
            .map(
                |fn_item| {
                    let fn_name = &fn_item.sig.ident;
                    let fn_args = fn_item.sig.inputs.iter().filter_map(|arg| {
                            if let syn::FnArg::Typed(pat_type) = arg {
                                Some(&pat_type.ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    let fn_ret = match &fn_item.sig.output {
                        syn::ReturnType::Default => quote!(()),
                        syn::ReturnType::Type(_, type_boxed) => quote!(#type_boxed),   
                    };

                    let mut msg = heck::AsUpperCamelCase(fn_name.to_string()).to_string();
                    
                    let act_attr = fn_item.attrs.iter().find(|attr| attr.path().is_ident("act")).unwrap();
                    if let Err(e) = act_attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("msg") {
                            let value;
                            parenthesized!(value in meta.input);
                            let value: Ident = value.parse()?;
                            let value = value.to_string();
                            msg = heck::AsUpperCamelCase(value).to_string();
                            return Ok(())
                        }

                        Err(meta.error("Unrecognized attribute"))
                    }) {
                        return (e.to_compile_error(), e.to_compile_error(), e.to_compile_error());
                    }

                    let enum_value = format_ident!("{}", msg);

                    let req = if fn_args.len() == 0 {
                        quote! {
                            #enum_value
                        }
                    } else {
                        quote! {
                            #enum_value(#(#fn_args),*)
                        }
                    };

                    let resp = quote! {
                        #enum_value(#fn_ret)
                    };

                    let unpack = match fn_args.len() {
                        0 => quote!(#actor_req_name::#enum_value => #actor_resp_name::#enum_value(self.#fn_name().await)),
                        _ => {
                            let unpack: Vec<_> = fn_args.iter().enumerate().map(|(i, _)| {
                                    let i = syn::Index::from(i);
                                    let unpackvar = format_ident!("req{}", i);
                                    quote!(#unpackvar)
                                })
                                .collect();
                            quote!(#actor_req_name::#enum_value(#(#unpack),*) => #actor_resp_name::#enum_value(self.#fn_name(#(#unpack),*).await))
                        },
                    };

                    (req, resp, unpack)
                }
            )
            .collect::<Vec<_>>();

        let req_enum_values = msg_enum_values.iter().map(|(req, _, _)| req);
        let resp_enum_values = msg_enum_values.iter().map(|(_, resp, _)| resp);
        let unpack_enum_values = msg_enum_values.iter().map(|(_, _, unpack)| unpack);

        let mut impl_item = impl_item.clone();

        impl_item.items.iter_mut()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(fn_item) => 
                    if fn_item.attrs.iter().any(|attr| attr.path().is_ident("act")) {
                        Some(fn_item)
                    }
                    else {
                        None
                    },
                _ => None,
            })
            .for_each(|fn_item| fn_item.attrs.retain(|attr| ! attr.path().is_ident("act")));

        let output = quote! {
            #struct_item

            #impl_item

            #[derive(Debug)]
            #vis enum #actor_req_name {
                #(#req_enum_values),*
            }

            #[derive(Debug, PartialEq)]
            #vis enum #actor_resp_name {
                #(#resp_enum_values),*
            }

            #[absinthe::prelude::async_trait]
            impl #generics absinthe::Actor for #actor_name #generic_params
            #where_clause
            {
                type Request  = #actor_req_name;
                type Response = #actor_resp_name;

                async fn recv_msg(&mut self, req: Self::Request) -> Self::Response {
                    match req {
                        #(#unpack_enum_values),*
                    }
                }
            }
        };

        output.to_tokens(tokens);
    }
}