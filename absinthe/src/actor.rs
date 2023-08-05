use crate::dev::prelude::*;



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

#[async_trait]
pub trait Actor   : Send + Sync + 'static {
    type Request  : Send;
    type Response : Send;

    async fn recv_msg(&mut self, req: Self::Request) -> Self::Response;
}



pub struct ActorHandle<A> 
    where A: Actor,
{
    pub task: TaskHandle<()>,
    tx: Tx<(A::Request, Option<Tx1<A::Response>>)>,
}

impl<A> Drop for ActorHandle<A> 
    where A: Actor,
{
    fn drop(&mut self) {
        let _ = self.tx.close();
    }
}

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
