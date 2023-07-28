mod dev;
mod msg;


use dev::prelude::*;



#[proc_macro]
pub fn send(input: TokenStream) -> TokenStream {
    let MsgSend { actor, payload } = parse_macro_input!(input as MsgSend);

    let payload = if payload.len() == 0 {
        quote!(())
    } else {
        quote!((#(#payload),*))
    };

    let expanded = quote! {
        #actor.send_msg(#payload)
    };

    expanded.into()
}

#[proc_macro]
pub fn notify(input: TokenStream) -> TokenStream {
    let MsgSend { actor, payload } = parse_macro_input!(input as MsgSend);

    let payload = if payload.len() == 0 {
        quote!(())
    } else {
        quote!((#(#payload),*))
    };

    let expanded = quote! {
        #actor.notify_msg(#payload)
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn actor_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    if input_fn.sig.asyncness.is_none() {
        return syn::Error::new_spanned(input_fn.sig.fn_token, "The `actor_fn` attribute can only be used on async functions!")
            .to_compile_error()
            .into();
    }

    for arg in &input_fn.sig.inputs {
        if let syn::FnArg::Receiver(_) = arg {
            return syn::Error::new_spanned(arg, "The `actor_fn` attribute cannot be used on associated functions or struct methods!")
                .to_compile_error()
                .into();
        }
    }

    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_ret = match &input_fn.sig.output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, type_boxed) => quote!(#type_boxed),   
    };
    let fn_generics = &input_fn.sig.generics;

    let actor_struct_name = format_ident!("{}ActorFn", heck::AsUpperCamelCase(&fn_name.to_string()).to_string());
    let fn_args_types: Vec<_> = fn_args.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            Some(&pat_type.ty)
        } else {
            None
        }
    }).collect();

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

    let fn_args = match fn_args.len() {
        0 => quote!(()),
        1 => quote!(#(#fn_args_types),*),
        _ => quote!((#(#fn_args_types),*)),
    };

    

    let output = quote! {
        struct #actor_struct_name;

        impl #actor_struct_name {
            fn new() -> Self {
                Self
            }
    
            #input_fn
        }

        #[absinthe::prelude::async_trait]
        impl Actor for #actor_struct_name {
            type Request  = #fn_args;
            type Response = #fn_ret;

            async fn recv_msg(&self, req: Self::Request) -> Self::Response {
                Self::#fn_name(#fn_args_unpack).await
            }
        }
    
        fn #fn_name #fn_generics () -> absinthe::actor::ActorHandle<#actor_struct_name> {
            let actor = #actor_struct_name::new();
            absinthe::spawn(actor)
        }
    };

    output.into()
}
