use crate::dev::prelude::*;
use crate::model::test::TestFnAttrModel;
use crate::model::test::TestFnModel;

impl Parse for TestFnAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}

impl Parse for TestFnModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}