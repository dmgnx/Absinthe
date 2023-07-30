mod dev;
mod msg;
mod actorizer;


use actorizer::function::ActorizeFn;
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
pub fn actor(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_act = input.clone();
    let input_act = parse_macro_input!(input_act as Item);

    match &input_act {
        Item::Fn(_) => {
            actor_fn(_attr, input)
        },
        _ => {
            return syn::Error::new_spanned(input_act, "The `actor` attribute could not resolve the following block as an actor!")
                .to_compile_error()
                .into();
        }
    }
}

#[proc_macro_attribute]
pub fn actor_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as ActorizeFn).into()
}
