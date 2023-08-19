mod main;
mod test;
mod actor;
mod send;
mod notify;

pub use main::*;
pub use test::*;
pub use actor::*;
pub use send::*;
pub use notify::*;

use crate::dev::prelude::*;

pub trait ICodeGen {
    type AttrModel;
    type Model;

    fn codegen(attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream;
}

pub struct NoAttrModel;

impl Parse for NoAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}

pub struct CodeGen;

impl CodeGen {
    pub fn codegen<Gen>(attr: Option<TokenStream>, input: TokenStream) -> TokenStream
    where
        Gen: ICodeGen,
        Gen::AttrModel: Parse,
        Gen::Model: Parse,
    {
        let attr = if attr.is_some() {
            match syn::parse2::<Gen::AttrModel>(attr.unwrap()) {
                Ok(attr) => Some(attr),
                Err(err) => return err.to_compile_error().into(),
            }
        } else {
            None
        };

        let model = match syn::parse2::<Gen::Model>(input) {
            Ok(model) => model,
            Err(err) => return err.to_compile_error().into(),
        };

        Gen::codegen(attr, model)
    }
}