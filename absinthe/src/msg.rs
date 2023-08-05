use crate::dev::prelude::*;

/// Courier trait, used to interact with actors.
/// Required to use the send! and notify! macros.
#[async_trait]
pub trait Courier<A>
where
    A: Actor,
{    
    /// Send a message to an actor, and wait for a response.
    async fn send_msg(&self, req: A::Request) -> A::Response;

    /// Send a message to an actor, and don't wait for a response.
    async fn notify_msg(&self, req: A::Request);
}
