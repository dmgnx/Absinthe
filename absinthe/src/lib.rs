//! # Absinthe
//! 
//! Absinthe is a library that allows you to create actors in Rust, and communicate with them using async messages.
//! It provides Actor Model primitives, and a super-macro to easily create actors.
//! Just write your Rust code, and Absinthe will handle the rest.
//! 
//! ## Features
//! 
//! - **Actor Model** - Absinthe provides an Actor trait, and a super-macro to easily create actors.
//! - **Async Messaging** - Absinthe provides a Courier trait, and macros to easily send messages to actors.
//! - **Async Tasks** - Absinthe provides a spawn function to spawn actors as async tasks.
//! - **Async Channels** - Absinthe uses async channels to communicate with actors.
//! - **Actor Functions** - actor! macro can be used to 'actorize' functions, implementing the Actor trait and spawning constructors.
//! - **Actor Structs** - actor! macro can be used to 'actorize' structs, implementing the Actor trait and message routing.
//!
//! ## Example
//! ```rust
//! use absinthe::prelude::*;
//! 
//! actor! {
//!     async fn add(a: u32, b: u32) -> u32 {
//!        a + b
//!    }
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!    let adder = add();
//!    assert_eq!(send!(adder, 1, 2).await, 3);
//! }
//! ```
//! 

/// Only used for Absinthe's own development.
mod dev;

/// Absinthe prelude, providing a good starter pack to start writing actors.
pub mod prelude;

pub mod error;
pub mod supervisor;

/// Actor traits & primitives.
pub mod actor;



/// Message traits & primitives.
pub mod courier;

pub use crate::error::Error;
pub use crate::actor::Actor;
pub use crate::courier::Courier;

/// Provides the actor! macro, which 'actorizes' functions & structs.
/// Also provides messaging macros, such as send! and notify!
pub use absinthe_macros::*;
