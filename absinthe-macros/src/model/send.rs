use crate::dev::prelude::*;

pub struct SendModel {
    pub actor: Expr,
    pub payload: Vec<Expr>,
}