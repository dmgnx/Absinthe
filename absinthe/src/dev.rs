pub mod prelude {
    pub use async_trait::async_trait;
    
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
}

#[cfg(feature = "tokio-runtime")]
pub mod async_rt {
    pub use tokio::spawn as task;
    pub use tokio::task::JoinHandle as TaskHandle;
    pub use tokio::sync::oneshot::{channel as chan1, Sender as Tx1, Receiver as Rx1};
}

#[cfg(feature = "async-std-runtime")]
pub mod async_rt {
    pub use async_std::task::spawn as task;
    pub use async_std::task::JoinHandle as TaskHandle;
    pub use futures::channel::oneshot::{channel as chan1, Sender as Tx1, Receiver as Rx1};   
}
