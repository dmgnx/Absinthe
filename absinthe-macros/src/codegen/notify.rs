use crate::dev::prelude::*;

use crate::model::{notify::*, NoAttrModel};

use super::ICodeGen;

pub struct NotifyCodeGen;

impl ICodeGen for NotifyCodeGen {
    type AttrModel = NoAttrModel;
    type Model = NotifyModel;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream {
        quote!()
    }
}
