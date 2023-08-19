use crate::dev::prelude::*;
use super::model::*;

impl Parse for ActorAttrModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attr = Self {
            name: None,
        };

        while ! input.is_empty() {
            let name = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;

            match name.to_string().as_str() {
                "name" => {
                    let name = input.parse::<Ident>()?;
                    attr.name = Some(name);
                },
                _ => return Err(input.error("Expected `name`")),
            }

            if input.is_empty() {
                break;
            }
            else {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(attr)
    }
}

impl Parse for ActorModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let actor_fn = input.parse::<ItemFn>()?;

        if actor_fn.sig.asyncness.is_none() {
            return Err(input.error("Expected async function"));
        }

        let vis = actor_fn.vis.clone();
        let generics = actor_fn.sig.generics.clone();
        let name = actor_fn.sig.ident.clone();

        let state_struct = actor_fn.block.stmts
            .iter()
            .find_map(
                |stmt| match stmt {
                    Stmt::Item(Item::Struct(state_struct)) => if state_struct.ident == "State" {
                            Some(state_struct.clone())
                        } else {
                            None
                        },
                    _ => None,
                });
        
        let req_t = actor_fn.sig.inputs
            .iter()
            .filter_map(
                |arg| match arg {
                    FnArg::Typed(PatType { pat, ty, .. }) => match pat.as_ref() {
                        Pat::Ident(PatIdent { ident, .. }) => if ident == "self" {
                                None
                            } else {
                                Some(ty.clone())
                            },
                        _ => None,
                    },
                    _ => None,
                })
            .fold(Punctuated::<Type, Token![,]>::new(), |mut acc, ty| {
                acc.push(*ty);
                acc
            });
        let req_t = match req_t.len() {
            1 => req_t.first().unwrap().clone(),
            _ => Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: req_t,
            }),  
        };

        let resp_t = match &actor_fn.sig.output {
            ReturnType::Default => Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: Punctuated::<Type, Token![,]>::new(),
            }),
            ReturnType::Type(_, ty) => (**ty).clone(),
        };
    
        Ok(Self {            
            vis,
            generics,
            name,

            actor_fn,
            state_struct,
            
            req_t,
            resp_t,
        })
    }
}
