use crate::dev::prelude::*;


mod model {
    use crate::dev::prelude::*;

    pub struct OneshotModel(pub SendModel);
}

mod parser {
    use crate::dev::prelude::*;

    impl Parse for OneshotModel {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let model = input.parse::<SendModel>()?;

            Ok(OneshotModel(model))
        }
    }
}

pub use model::*;

use super::ICodeGen;

pub struct OneshotCodeGen;

impl ICodeGen for OneshotCodeGen {
    type AttrModel = NoAttrModel;
    type Model = OneshotModel;

    fn codegen(_attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream {
        let OneshotModel(mut model) = model;

        let actor_ty = model.courier;

        model.courier = parse_quote! { courier };
        let send = SendCodeGen::codegen(None, model);

        quote!{
            async {
                let mut supv = Supervisor::new();

                let res = match supv.spawn::<#actor_ty>("_").await {
                    Ok(courier) => {
                        let resp = #send.await;
                        Ok(resp)
                    },
                    Err(err) => Err(err),
                };

                supv.stop().await;
                res
            }
        }
    }
}
