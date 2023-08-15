use crate::dev::prelude::*;
use crate::model::notify::NotifyModel;

impl Parse for NotifyModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}