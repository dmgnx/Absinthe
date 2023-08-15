use crate::dev::prelude::*;

use crate::model::{send::*, NoAttrModel};

use super::ICodeGen;

pub struct SendCodeGen;

impl ICodeGen for SendCodeGen {
    type AttrModel = NoAttrModel;
    type Model = SendModel;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream {
        quote!()
    }
}
