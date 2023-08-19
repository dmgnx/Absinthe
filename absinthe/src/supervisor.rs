use crate::dev::prelude::*;
use crate::error;

use std::any::Any;
use std::collections::HashMap;
use petgraph::graph::DiGraph;

enum ActorCmd {
    Stop,
}

trait IActorHandle
{
    fn abort(&self);
    fn stop(&self);
    fn as_any(&self) -> &dyn Any;
}

struct ActorHandle<A> 
where 
    A: Actor
{
    pub task: TaskHandle<()>,
    pub tx: Tx<(A::Request, Option<Tx1<A::Response>>)>,
    pub cmd_tx: Tx<ActorCmd>,
}

impl <A> IActorHandle for ActorHandle<A> 
where 
    A: Actor
{
    fn abort(&self) {
        self.task.abort();
    }

    fn stop(&self) {
        let _ = self.cmd_tx.send(ActorCmd::Stop);
        self.cmd_tx.close();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub struct Supervisor {
    links: DiGraph<String, String>,
    actors: HashMap<String, Box<dyn IActorHandle>>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            links: DiGraph::new(),
            actors: HashMap::new(),
        }
    }

    #[tracing::instrument(skip(self, actor))]
    pub fn deploy<A>(&mut self, actor: A, address: &str) -> Result<ActorCourier<A>, Error>
    where
        A: Actor
    {
        let (cmd_tx, cmd_rx) = chan::<ActorCmd>();
        let (tx, rx) = chan::<(A::Request, Option<Tx1<A::Response>>)>();
        
        let address1 = address.to_owned();

        let task = task(async move {
            info!("Actor {} started", address1);
            let mut actor = actor;
            loop {
                select! {
                    cmd = cmd_rx.recv() => match cmd {
                        Ok(ActorCmd::Stop) => {
                            info!("Actor {} stopped", address1);
                            break;
                        }
                        Err(_) => {
                            error!("Actor {} command channel closed", address1);
                            break;
                        }
                    },
                    msg = rx.recv() => match msg {
                        Ok((req, tx1resp)) => {
                            debug!("Actor {} received message", address1);
                            let resp = actor.receive(req).await;
                            if let Some(tx1resp) = tx1resp {
                                let _ = tx1resp.send(resp);
                            }
                        },
                        Err(_) => {
                            warn!("Actor {} channel closed", address1);
                            break;
                        }
                    }
                }
            }
        });

        let actor = Box::new(ActorHandle::<A> {
            task,
            tx: tx.clone(),
            cmd_tx,
        });

        self.actors.insert(address.to_owned(), actor);

        Ok(ActorCourier::new(tx))
    }

    #[tracing::instrument(skip(self))]
    pub async fn spawn<A>(&mut self, address: &str) -> Result<ActorCourier<A>, Error>
    where
        A: Actor
    {
        let actor = match A::start().await {
            Ok(actor) => actor,
            Err(err) => {
                error!("Actor {} failed to start: {}", address, err);
                return Err(err);
            }
        };

        self.deploy(actor, address)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get<A>(&mut self, address: &str) -> Result<ActorCourier<A>, Error>
    where
        A: Actor
    {
        if let Some(courier) = self.find::<A>(address) {
            Ok(courier)
        } else {
            self.spawn(address).await
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn find<A>(&self, address: &str) -> Option<ActorCourier<A>>
    where
        A: Actor
    {
        if let Some(actor) = self.actors.get(address) {
            let actor = match actor.as_any().downcast_ref::<ActorHandle<A>>() {
                Some(actor) => actor,
                None => {
                    return None;
                }
            };

            let tx = actor.tx.clone();
            Some(ActorCourier::new(tx))
        } else {
            None
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn stop(&self) {
        for actor in self.actors.iter() {
            let (_, handle) = actor;
            handle.stop();
        }
    }

    #[tracing::instrument(skip(self))]
    fn abort(&mut self) {
        for actor in self.actors.iter() {
            let (_, handle) = actor;
            handle.abort();
        }
        self.actors.clear();
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        self.abort();
    }
}