use crate::dev::prelude::*;
use crate::model::main::MainFnAttrModel;
use crate::model::main::MainFnModel;

impl Parse for MainFnAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}

impl Parse for MainFnModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}