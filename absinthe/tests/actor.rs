use absinthe::{prelude::*, actor::ActorClosure};

#[tokio::test]
async fn test_echo_actor() {
    #[absinthe::actor]
    async fn echo<T>(&self, msg: T) -> T {
        msg
    }
    
    let mut supv = Supervisor::new();
    let echo = supv.spawn::<EchoActor<u32>>("echo").await.unwrap();

    assert_eq!(send!(echo, 1).await, 1);
}


#[tokio::test]
async fn test_actor_fn() {
    #[absinthe::actor]
    async fn add(a: u32, b: u32) -> u32 {
        a + b
    }
    
    let sum = oneshot!(AddActor, 1, 2).await.unwrap();

    assert_eq!(sum, 3);
}

#[tokio::test]
async fn test_actor_fn_with_state() {
    #[absinthe::actor]
    async fn counter(&mut self) -> u32 {
        struct State {
            #[init(0)]
            count: u32
        }
        
        self.count += 1;
        self.count
    }

    let mut supv = Supervisor::new();
    let counter = supv.spawn::<CounterActor>("counter").await.unwrap();
    
    let count = send!(counter).await;
    assert_eq!(count, 1);

    let _ = send!(counter).await;
    let count = send!(counter).await;
    assert_eq!(count, 3);
}

#[tokio::test]
async fn test_generic_actor_fn_with_state() {
    #[absinthe::actor]
    async fn counter<T: std::ops::Add<Output = T> + Default + Copy>(&mut self, inc: T) -> T {
        struct State<T> {
            #[init(T::default())]
            count: T
        }
        
        self.count = self.count + inc;
        self.count
    }

    let mut supv = Supervisor::new();
    let counter = supv.spawn::<CounterActor<u128>>("counter").await.unwrap();
    
    let count = send!(counter, 5).await;
    assert_eq!(count, 5);

    let _ = send!(counter, 2).await;
    let count = send!(counter, 4).await;
    assert_eq!(count, 11);
}

#[tokio::test]
async fn test_get_actor_by_address() {
    #[absinthe::actor]
    async fn counter(&mut self) -> u32 {
        struct State {
            #[init(0)]
            count: u32
        }
        
        self.count += 1;
        self.count
    }

    let mut supv = Supervisor::new();
    let counter = supv.get::<CounterActor>("counter").await.unwrap();
    
    let count = send!(counter).await;
    assert_eq!(count, 1);

    let counter = supv.get::<CounterActor>("counter").await.unwrap();
    let _ = send!(counter).await;
    let count = send!(counter).await;
    assert_eq!(count, 3);

    let counter = supv.get::<CounterActor>("counter1").await.unwrap();
    let _ = send!(counter).await;
    let count = send!(counter).await;
    assert_eq!(count, 2);
}


#[tokio::test]
async fn test_actor_closure() {
    let add = ActorClosure::new(|msg: (u32, u32)| -> u32 {
        let (a, b) = msg;
        a + b
    });

    let mut supv = Supervisor::new();
    let add = supv.deploy(add, "add").unwrap();

    assert_eq!(send!(add, (1, 2)).await, 3);
}

// #[tokio::test]
// async fn test_actor_fn_with_state_from_config() {
//     #[absinthe::actor]
//     async fn counter(self) -> u32 {
//         struct State {
//             #[prop("counter.{address}.count")]
//             count: u32
//         }
        
//         self.count += 1
//     }

//     let config = spawn!(supv, ConfigActor).await;
//     notify!(config, Prop::Int("counter.test.count", 0)).await;

//     let counter = spawn!(supv, CounterActor, "test").await;
    
//     assert_eq!(send!(counter, ()).await, 1);
//     assert_eq!({
//         let _ = send!(counter, ()).await;
//         send!(counter, ()).await
//     }, 3);
// }


// #[tokio::test]
// async fn test_mock_actor_fn(supv: Supervisor) {
//     enum Msg {
//         Ping,
//         Pong,
//     }

//     #[absinthe::actor(name = MyPingActor)]
//     async fn ping(self, msg: Msg) -> Msg {
//         struct Context {
//             pong: ActorCourier<PongActor>
//         }
        
//         send!(self.pong, msg).await
//     }

//     #[absinthe::actor(name = MyPongActor)]
//     async fn pong(msg: Msg) -> Msg {
//         match msg {
//             Msg::Ping => Msg::Pong,
//             Msg::Pong => Msg::Ping,
//         }
//     }

//     mock!(supv, MyPongActor, |_msg| Msg::Ping);

//     let ping = oneshot!(supv, MyPingActor, Msg::Ping).await;

//     assert_eq!(ping, Msg::Ping);
// }

