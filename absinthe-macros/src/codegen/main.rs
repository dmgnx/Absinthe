use crate::dev::prelude::*;

use crate::model::main::*;

use super::ICodeGen;

pub struct MainFnCodeGen;

impl ICodeGen for MainFnCodeGen {
    type AttrModel = MainFnAttrModel;
    type Model = MainFnModel;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream {
        quote!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::prelude::*;

    #[test]
    fn test_main_fn_codegen() {
    let attr = quote!{};
    let input = quote! {
        async fn main(supv: Supervisor) {
            println!("Hello world!");
        }
    };

    let expected = quote! {
        #[tokio::main]
        async fn main() {
            async fn __start(supv: Supervisor) {
                println!("Hello world!");
            }

            System::new().run(|system| async move {
                let supv = system.supervisor("main");

                __start(supv).await;
            }).await;
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<MainFnAttrModel, MainFnModel, MainFnCodeGen>(Some(attr), input).to_string()
    )
}
}