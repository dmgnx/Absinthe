use crate::dev::prelude::*;

use crate::model::actor_impl::*;

use super::ICodeGen;

pub struct ActorImplCodeGen;

impl ICodeGen for ActorImplCodeGen {
    type AttrModel = ActorImplAttrModel;
    type Model = ActorImplModel;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream {
        quote!()
    }
}
