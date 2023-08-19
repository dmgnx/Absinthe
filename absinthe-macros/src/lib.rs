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

mod codegen;

use proc_macro::TokenStream;

use crate::codegen::*;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    CodeGen::codegen::<MainFnCodeGen>(Some(attr.into()), input.into()).into()
}

#[proc_macro_attribute]
pub fn test(attr: TokenStream, input: TokenStream) -> TokenStream {
    CodeGen::codegen::<TestFnCodeGen>(Some(attr.into()), input.into()).into()
}

#[proc_macro_attribute]
pub fn actor(attr: TokenStream, input: TokenStream) -> TokenStream {
    CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).into()
}

/// Send a message to an actor, and wait for a response.
#[proc_macro]
pub fn send(input: TokenStream) -> TokenStream {
    CodeGen::codegen::<SendCodeGen>(None, input.into()).into()
}

/// Send a message to an actor, and don't wait for a response.
#[proc_macro]
pub fn notify(input: TokenStream) -> TokenStream {
    CodeGen::codegen::<NotifyCodeGen>(None, input.into()).into()
}

