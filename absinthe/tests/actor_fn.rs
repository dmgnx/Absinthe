use absinthe::prelude::*;

#[tokio::test]
async fn basic_function_fn_add() {
    #[absinthe::actor_fn]
    async fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    let actor = add();

    assert_eq!(absinthe::send!(actor, 3, 2).await, 5);
}

#[tokio::test]
async fn basic_function_fn_concat() {
    #[absinthe::actor_fn]
    async fn concat(mut a: String, b: String) -> String {
        a += &b;
        a
    }

    let actor = concat();

    assert_eq!(absinthe::send!(actor, "Hello, ".to_string(), "world!".to_string()).await, "Hello, world!");
}

#[tokio::test]
async fn return_tuple() {
    #[absinthe::actor_fn]
    async fn swap(a: u32, b: u32) -> (u32, u32) {
        (b, a)
    }

    let actor = swap();

    assert_eq!(absinthe::send!(actor, 3, 2).await, (2, 3));
}

#[tokio::test]
async fn return_unit() {
    #[absinthe::actor_fn]
    async fn print(a: u32) {
        format!("{}", a);
    }

    let actor = print();

    assert_eq!(absinthe::send!(actor, 3).await, ());
}

#[tokio::test]
async fn no_arg() {
    #[absinthe::actor_fn]
    async fn print() {
        format!("Hello, world!");
    }

    let actor = print();

    assert_eq!(absinthe::send!(actor).await, ());
}

#[tokio::test]
async fn ping_pong() {
    #[absinthe::actor_fn]
    async fn ping(count: u8, pong: ActorHandle<PongActorFn>) -> String {
        for _ in 0..(count - 1) {
            absinthe::notify!(pong).await;
        }

        absinthe::send!(pong).await
    }

    #[absinthe::actor_fn]
    async fn pong() -> String {
        "pong".to_string()
    }

    let ping = ping();
    let pong = pong();

    assert_eq!(absinthe::send!(ping, 8, pong).await, "pong");
}

#[tokio::test]
async fn generic() {
    // #[absinthe::actor_fn]
    // async fn add<T: std::ops::Add<Output = T> + Copy>(a: T, b: T) -> T {
    //     a + b
    // }

    struct AddActorFn<T:std::ops::Add<Output = T> + Send + Sync + 'static> {
        _marker: std::marker::PhantomData<T>,
    }
    
    impl<T:std::ops::Add<Output = T> + Send + Sync + 'static> AddActorFn<T> {
      fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
      }
      async fn add(a:T,b:T) -> T {
        a+b
      }
    }

    #[absinthe::prelude::async_trait]
    impl<T:std::ops::Add<Output = T> + Send + Sync + 'static> Actor for AddActorFn<T> {
      type Request = (T,T);
      type Response = T;

      async fn recv_msg(&self,req:Self::Request) -> Self::Response {
        Self::add(req.0,req.1).await
      }
    }
    
    fn add<T:std::ops::Add<Output = T> + Send + Sync + 'static>() -> absinthe::actor::ActorHandle<AddActorFn<T>>{
      let actor = AddActorFn::new();
      absinthe::spawn(actor)
    }

    let add_u8 = add::<u8>();
    let add_u32 = add::<u32>();
    let add_f64 = add::<f64>();


    assert_eq!(absinthe::send!(add_u8, 3, 2).await, 5u8);
    assert_eq!(absinthe::send!(add_u32, 3, 2).await, 5u32);
    assert_eq!(absinthe::send!(add_f64, 3.0, 2.0).await, 5.0f64);
}