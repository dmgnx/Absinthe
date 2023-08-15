mod main;
mod test;
mod actor_fn;
mod actor_impl;
mod send;
mod notify;

pub use main::*;
pub use test::*;
pub use actor_fn::*;
pub use actor_impl::*;
pub use send::*;
pub use notify::*;

use crate::dev::prelude::*;

pub trait ICodeGen {
    type AttrModel;
    type Model;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream;
}

pub struct CodeGen;

impl CodeGen {
    pub fn codegen<AttrModel, Model, Gen>(attr: Option<TokenStream>, input: TokenStream) -> TokenStream
    where
        AttrModel: Parse,
        Model: Parse,
        Gen: ICodeGen<AttrModel = AttrModel, Model = Model>,
    {
        let attr = if attr.is_some() {
            match syn::parse2::<AttrModel>(attr.unwrap()) {
                Ok(attr) => Some(attr),
                Err(err) => return err.to_compile_error().into(),
            }
        } else {
            None
        };

        let model = match syn::parse2::<Model>(input) {
            Ok(model) => model,
            Err(err) => return err.to_compile_error().into(),
        };

        Gen::codegen(&attr, &model)
    }
}