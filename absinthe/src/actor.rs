use crate::dev::prelude::*;



pub fn spawn<A>(actor: A) -> ActorHandle<A>
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

    async fn recv_msg(&self, req: Self::Request) -> Self::Response;
}



pub struct ActorHandle<A> 
    where A: Actor,
{
    task: TaskHandle<()>,
    tx: Tx<(A::Request, Option<Tx1<A::Response>>)>,
}

impl<A> ActorHandle<A> 
    where A: Actor,
{
    pub async fn send_msg(&self, req: A::Request) -> A::Response {
        let (tx1, rx1) = chan1();
        let _ = self.tx.send((req, Some(tx1))).await;
        rx1.await.unwrap()
    }

    pub async fn notify_msg(&self, req: A::Request) {
        let _ = self.tx.send((req, None)).await;
    }
}

