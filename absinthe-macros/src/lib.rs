mod dev;
mod msg;
mod actorizer;


use actorizer::function::ActorizeFn;
use actorizer::structure::ActorizeStruct;
use dev::prelude::*;
use syn::File;



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

#[proc_macro]
pub fn actor(input: TokenStream) -> TokenStream {
    let section = input.clone();
    let section = parse_macro_input!(section as File);

    match section.items.first().unwrap() {
        Item::Struct(_) => {
            parse_macro_input!(input as ActorizeStruct).into()
        },
        Item::Fn(_) => {
            parse_macro_input!(input as ActorizeFn).into()

        },
        _ => syn::Error::new_spanned::<TokenStream2, _>(input.into(), "Expected `struct` or `fn`").to_compile_error().into()
    }
}