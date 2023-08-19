use crate::dev::prelude::*;

mod model {
    use crate::dev::prelude::*;

    pub struct MainFnAttrModel {

    }

    pub struct MainFnModel {

    }
}

mod parser {
    use crate::dev::prelude::*;

    impl Parse for MainFnAttrModel {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self{})
        }
    }
    
    impl Parse for MainFnModel {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self{})
        }
    }
}

pub use model::*;

use super::ICodeGen;

pub struct MainFnCodeGen;

impl ICodeGen for MainFnCodeGen {
    type AttrModel = MainFnAttrModel;
    type Model = MainFnModel;

    fn codegen(attr: Option<Self::AttrModel>, model: Self::Model) -> TokenStream {
        quote!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        CodeGen::codegen::<MainFnCodeGen>(Some(attr), input).to_string()
    )
}
}