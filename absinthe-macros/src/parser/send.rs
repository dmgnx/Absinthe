use crate::dev::prelude::*;
use crate::model::send::SendModel;

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