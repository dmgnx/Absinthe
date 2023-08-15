use crate::dev::prelude::*;

use crate::model::test::*;

use super::ICodeGen;

pub struct TestFnCodeGen;

impl ICodeGen for TestFnCodeGen {
    type AttrModel = TestFnAttrModel;
    type Model = TestFnModel;

    fn codegen(attr: &Option<Self::AttrModel>, model: &Self::Model) -> TokenStream {
        quote!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::prelude::*;

    #[test]
    fn test_test_fn() {
        let attr = quote!{};
        let input = quote! {
            async fn test_my_actor(supv: Supervisor) {
                assert_eq!(1, 1);
            }
        };

        let expected = quote! {
            #[tokio::test]
            async fn test_my_actor() {
                System::new().run(|sys| async move {
                    let supv = sys.supervisor("test_my_actor");

                    async fn run_test(supv: Supervisor) {
                        assert_eq!(1, 1);
                    }

                    run_test(supv).await;
                }).await;
            }
        };

        assert_eq!(
            expected.to_string(),
            CodeGen::codegen::<TestFnAttrModel, TestFnModel, TestFnCodeGen>(Some(attr.into()), input.into()).to_string()
        )
    }
}