use crate::dev::prelude::*;


mod model {
    use crate::dev::prelude::*;

    pub struct NotifyModel {

    }
}

mod parser {
    use crate::dev::prelude::*;

    impl Parse for NotifyModel {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self{})
        }
    }
}

pub use model::*;

use super::ICodeGen;

pub struct NotifyCodeGen;

impl ICodeGen for NotifyCodeGen {
    type AttrModel = NoAttrModel;
    type Model = NotifyModel;

    fn codegen(attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream {
        quote!()
    }
}
