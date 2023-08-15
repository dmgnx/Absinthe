use crate::dev::prelude::*;

use crate::model::actor_fn::*;

use super::ICodeGen;

pub struct ActorFnCodeGen;

impl ICodeGen for ActorFnCodeGen {
    type AttrModel = ActorFnAttrModel;
    type Model = ActorFnModel;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream {
        quote!()
    }
}
