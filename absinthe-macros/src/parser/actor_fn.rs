use crate::dev::prelude::*;
use crate::model::actor_fn::ActorFnAttrModel;
use crate::model::actor_fn::ActorFnModel;

impl Parse for ActorFnAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}

impl Parse for ActorFnModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{})
    }
}