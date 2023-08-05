//! # Absinthe Macros
//! 
//! Absinthe Macros is a library that provides macros for Absinthe.
//! 
//! ## Features
//! 
//! - **actor!** - The actor! macro can be used to 'actorize' functions & structs.
//! - **send!** - The send! macro can be used to send a message to an actor, and wait for a response.
//! - **notify!** - The notify! macro can be used to send a message to an actor, and don't wait for a response.
//! 

/// Module used for Absinthe's own development.
mod dev;

/// send! and notify! argument parsing.
mod msg;

/// actor! parsing and actorization.
mod actorizer;


use actorizer::function::ActorizeFn;
use actorizer::structure::ActorizeStruct;
use dev::prelude::*;
use syn::File;


/// Send a message to an actor, and wait for a response.
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

/// Send a message to an actor, and don't wait for a response.
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

/// Create an actor.
/// Can be used on a struct or a function.
/// If actorizing a struct, the struct must have an `impl` block.
/// The `impl` block must have the same name as the struct.
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