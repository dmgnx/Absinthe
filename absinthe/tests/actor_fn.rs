use absinthe::prelude::*;

#[tokio::test]
async fn basic_function_fn_add() {
    actor! {
        async fn add(a: u32, b: u32) -> u32 {
            a + b
        }
    }
    
    let actor = add();

    assert_eq!(absinthe::send!(actor, 3, 2).await, 5);
}

#[tokio::test]
async fn basic_function_fn_concat() {
    actor! {
        async fn concat(mut a: String, b: String) -> String {
            a += &b;
            a
        }
    }
    

    let actor = concat();

    assert_eq!(absinthe::send!(actor, "Hello, ".to_string(), "world!".to_string()).await, "Hello, world!");
}

#[tokio::test]
async fn return_tuple() {
    actor! {
        async fn swap(a: u32, b: u32) -> (u32, u32) {
            (b, a)
        }
    }
    
    let actor = swap();

    assert_eq!(absinthe::send!(actor, 3, 2).await, (2, 3));
}

#[tokio::test]
async fn return_unit() {
    actor! {
        async fn print(a: u32) {
            format!("{}", a);
        }
    }
    
    let actor = print();

    assert_eq!(absinthe::send!(actor, 3).await, ());
}

#[tokio::test]
async fn no_arg() {
    actor! {
        async fn print() {
            format!("Hello, world!");
        }
    }

    let actor = print();

    assert_eq!(absinthe::send!(actor).await, ());
}

#[tokio::test]
async fn ping_pong() {
    actor! {
        async fn ping(count: u8, pong: ActorHandle<PongActorFn>) -> String {
            for _ in 0..(count - 1) {
                absinthe::notify!(pong).await;
            }
    
            absinthe::send!(pong).await
        }
    }
    
    actor! {
        async fn pong() -> String {
            "pong".to_string()
        }
    }
    

    let ping = ping();
    let pong = pong();

    assert_eq!(absinthe::send!(ping, 8, pong).await, "pong");
}

#[tokio::test]
async fn generic() {
    actor! {
        async fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }
    }
    
    actor! {
        async fn sub<T> (a: T, b: T) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            a - b
        }
    }    

    let add_u8 = add::<u8>();
    let sub_u32 = sub::<u32>();
    let add_f64 = add::<f64>();


    assert_eq!(absinthe::send!(add_u8, 3, 2).await, 5u8);
    assert_eq!(absinthe::send!(sub_u32, 3, 2).await, 1u32);
    assert_eq!(absinthe::send!(add_f64, 3.0, 2.0).await, 5.0f64);
}
