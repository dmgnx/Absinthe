use crate::dev::prelude::*;


#[async_trait]
pub trait Courier<A>
where
    A: Actor,
{    
    async fn send_msg(&self, req: A::Request) -> A::Response;
    async fn notify_msg(&self, req: A::Request);
}
