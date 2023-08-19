pub mod prelude {
    pub use std::sync::Arc;

    pub use async_trait::async_trait;
    
    pub use tracing::*;

    pub use async_channel::{
        unbounded as chan,
        Sender as Tx,
        Receiver as Rx,   
    };

    pub use super::async_rt::*;

    pub use crate::prelude::*;

    pub use std::future::Future;
    pub use std::pin::Pin;
    pub use std::marker::PhantomData;
    pub use futures::future::FutureExt;
}

#[cfg(feature = "tokio-runtime")]
pub mod async_rt {
    pub use tokio::select as select;
    pub use tokio::spawn as task;
    pub use tokio::task::JoinHandle as TaskHandle;
    pub use tokio::sync::oneshot::{channel as chan1, Sender as Tx1, Receiver as Rx1};
}
