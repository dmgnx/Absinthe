use crate::dev::prelude::*;
use crate::model::actor_impl::ActorImplAttrModel;
use crate::model::actor_impl::ActorImplModel;

impl Parse for ActorImplAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}

impl Parse for ActorImplModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}