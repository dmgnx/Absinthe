use crate::dev::prelude::*;

mod model {
    use crate::dev::prelude::*;

    #[derive(Clone)]
    pub struct SendModel {
        pub courier: Expr,
        pub payload: Vec<Expr>,
    }
}

mod parser {
    use crate::dev::prelude::*;

    impl Parse for SendModel {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let courier: Expr = input.parse()?;
            let payload = match input.parse::<Token![,]>() {
                Ok(_) => {
                    input.parse_terminated(Expr::parse, Token!(,))?.into_iter().collect()
                },
                Err(_) => vec![],
            };
            
            Ok(SendModel { courier, payload })
        }
    }
}

pub use model::*;

use super::ICodeGen;

pub struct SendCodeGen;

impl ICodeGen for SendCodeGen {
    type AttrModel = NoAttrModel;
    type Model = SendModel;

    fn codegen(_attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream {
        let SendModel { courier, payload } = model;
        
        quote!{
            #courier.send((#(#payload),*))
        }
    }
}
