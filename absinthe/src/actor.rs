use crate::dev::prelude::*;

/// Spawn a new actor into a task, open a channel to send messages to the actor, and return a handle to it.
/// 
/// # Example
/// ```rust
/// use absinthe::prelude::*;
/// 
/// actor! {
///     struct Counter {
///         count: u32,
///     }
/// 
///     impl Counter {
///         fn new() -> Self {
///             Self {
///                 count: 0,
///             }
///         }
/// 
///         #[act(msg(inc))]
///         async fn increment(&mut self) -> u32 {
///             self.count += 1;
///             self.count
///         }
///     }
/// }
/// 
/// // Spawn the actor into a task, and open a channel to send messages to it.
/// spawn(Counter::new());
/// ```
pub fn spawn<A>(mut actor: A) -> ActorHandle<A>
where
    A: Actor,
{
    let (tx, rx) = chan::<(A::Request, Option<Tx1<A::Response>>)>();
    let task = task(async move {
        loop {
            match rx.recv().await {
                Ok((req, tx1resp)) => {
                    let resp = actor.recv_msg(req).await;
                    if let Some(tx1resp) = tx1resp {
                        let _ = tx1resp.send(resp);
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    });

    ActorHandle {
        task,
        tx,
    }
}

/// Actor trait, implemented by all actors.
#[async_trait]
pub trait Actor   : Send + Sync + 'static {
    /// The type of messages that this actor can receive.
    type Request  : Send;
    /// The type of responses that this actor can send.
    type Response : Send;

    /// Message handler, will generally be implemented as a message unpacker or router.
    async fn recv_msg(&mut self, req: Self::Request) -> Self::Response;
}

/// Actor handle, used to send messages to an actor.
pub struct ActorHandle<A> 
    where A: Actor,
{
    /// The task that the actor is running in. Type depends on your async runtime.
    pub task: TaskHandle<()>,
    tx: Tx<(A::Request, Option<Tx1<A::Response>>)>,
}

impl<A> Drop for ActorHandle<A> 
    where A: Actor,
{
    /// Close the channel to the actor, which will cause the actor to stop.
    fn drop(&mut self) {
        let _ = self.tx.close();
    }
}

/// Courier trait, implemented by all actors.
/// Used to send messages to actors.
/// Required to use the send! and notify! macros.
#[async_trait]
impl<A> Courier<A> for ActorHandle<A> 
    where A: Actor,
{
    async fn send_msg(&self, req: A::Request) -> A::Response {
        let (tx1, rx1) = chan1();
        let _ = self.tx.send((req, Some(tx1))).await;
        rx1.await.unwrap()
    }

    async fn notify_msg(&self, req: A::Request) {
        let _ = self.tx.send((req, None)).await;
    }
}
