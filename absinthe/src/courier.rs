use crate::dev::prelude::*;

/// Courier trait, used to interact with actors.
/// Required to use the send! and notify! macros.
#[async_trait]
pub trait ICourier<Request, Response>
where
    Request: Send,
    Response: Send
{    
    /// Send a message to an actor, and wait for a response.
    async fn send(&self, req: Request) -> Response;

    /// Send a message to an actor, and don't wait for a response.
    async fn notify(&self, req: Request);
}

#[derive(Clone)]
pub struct Courier<Request, Response> 
where
    Request: Send,
    Response: Send
{
    tx: Tx<(Request, Option<Tx1<Response>>)>,
}

impl<Request, Response> Courier<Request, Response>
where
    Request: Send,
    Response: Send
{
    pub(crate) fn new(tx: Tx<(Request, Option<Tx1<Response>>)>) -> Self {
        Self {
            tx,
        }
    }
}

#[async_trait]
impl<Request, Response> ICourier<Request, Response> for Courier<Request, Response>
where
    Request: Send,
    Response: Send
{
    #[tracing::instrument(skip(self, req))]
    async fn send(&self, req: Request) -> Response {
        let (tx1, rx1) = chan1();
        let _ = self.tx.send((req, Some(tx1))).await;
        rx1.await.unwrap()
    }

    #[tracing::instrument(skip(self, req))]
    async fn notify(&self, req: Request) {
        let _ = self.tx.send((req, None)).await;
    }
}

#[derive(Clone)]
pub struct ActorCourier<A>(Courier<A::Request, A::Response>)
where
    A: Actor;

impl<A> ActorCourier<A>
where
    A: Actor
{
    pub(crate) fn new(tx: Tx<(A::Request, Option<Tx1<A::Response>>)>) -> Self {
        Self(Courier::new(tx))
    }
}

impl <A> Into<Courier<A::Request, A::Response>> for ActorCourier<A>
where
    A: Actor
{
    fn into(self) -> Courier<A::Request, A::Response> {
        self.0
    }
}

#[async_trait]
impl<A> ICourier<A::Request, A::Response> for ActorCourier<A>
where
    A: Actor
{
    #[tracing::instrument(skip(self, req))]
    async fn send(&self, req: A::Request) -> A::Response {
        self.0.send(req).await
    }

    #[tracing::instrument(skip(self, req))]
    async fn notify(&self, req: A::Request) {
        self.0.notify(req).await
    }
}