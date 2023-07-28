use crate::dev::prelude::*;



pub struct MsgSend {
    pub actor: Expr,
    pub payload: Vec<Expr>,
}

impl Parse for MsgSend {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let actor: Expr = input.parse()?;
        let payload = match input.parse::<Token![,]>() {
            Ok(_) => {
                input.parse_terminated(Expr::parse, Token!(,))?.into_iter().collect()
            },
            Err(_) => vec![],
        };
        
        Ok(MsgSend { actor, payload })
    }
}