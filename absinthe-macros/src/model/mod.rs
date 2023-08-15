pub mod main;
pub mod test;
pub mod send;
pub mod notify;
pub mod actor_fn;
pub mod actor_impl;

pub use main::*;
pub use test::*;
pub use send::*;
pub use notify::*;
pub use actor_fn::*;
pub use actor_impl::*;

pub struct NoAttrModel;