mod main;
mod test;
mod send;
mod notify;
mod actor_fn;
mod actor_impl;

use crate::dev::prelude::*;
use crate::model::NoAttrModel;

impl Parse for NoAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}