use crate::dev::prelude::*;

pub struct ActorAttrModel {
    pub name: Option<Ident>,
}

pub struct ActorModel {
    pub vis: Visibility,
    pub generics: Generics,
    pub name: Ident,

    pub actor_fn: ItemFn,
    pub state_struct: Option<ItemStruct>,
    
    pub req_t: Type,
    pub resp_t: Type,
}
