use crate::dev::prelude::*;

/// Actor trait, implemented by all actors.
#[async_trait]
pub trait Actor   : Sized + Send + Sync + 'static 
{
    /// The type of messages that this actor can receive.
    type Request  : Send;
    /// The type of responses that this actor can send.
    type Response : Send;

    /// Message handler, will generally be implemented as a message unpacker or router.
    async fn receive(&mut self, req: Self::Request) -> Self::Response;
    
    async fn start() -> Result<Self, Error>;

    async fn stop(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ActorClosure<Request, Response>
where
    Request: Send + Sync + 'static,
    Response: Send + Sync + 'static
{
    handler: Box<dyn Fn(Request) -> Response + Send + Sync + 'static>
}

impl <Request, Response> ActorClosure<Request, Response>
where
    Request: Send + Sync + 'static,
    Response: Send + Sync + 'static
{
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static
    {
        Self {
            handler: Box::new(handler)
        }
    }
}

#[async_trait]
impl <Req, Resp> Actor for ActorClosure<Req, Resp> 
where
    Req: Send + Sync + 'static,
    Resp: Send + Sync + 'static
{
    type Request = Req;
    type Response = Resp;

    #[tracing::instrument(skip(self, req))]
    async fn receive(&mut self, req: Self::Request) -> Self::Response {
        (self.handler)(req)
    }

    #[tracing::instrument]
    async fn start() -> Result<Self, Error> {
        Err(Error::StartFailed)
    }
}