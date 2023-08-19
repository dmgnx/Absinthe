use crate::dev::prelude::*;

mod model {
    use crate::dev::prelude::*;

    pub struct SendModel {
        pub actor: Expr,
        pub payload: Vec<Expr>,
    }
}

mod parser {
    use crate::dev::prelude::*;

    impl Parse for SendModel {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let actor: Expr = input.parse()?;
            let payload = match input.parse::<Token![,]>() {
                Ok(_) => {
                    input.parse_terminated(Expr::parse, Token!(,))?.into_iter().collect()
                },
                Err(_) => vec![],
            };
            
            Ok(SendModel { actor, payload })
        }
    }
}

pub use model::*;

use super::ICodeGen;

pub struct SendCodeGen;

impl ICodeGen for SendCodeGen {
    type AttrModel = NoAttrModel;
    type Model = SendModel;

    fn codegen(attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream {
        quote!()
    }
}
