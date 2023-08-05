# Absinthe

*Are you an Actor & Async junkie? Then your app needs some Absinthe!*

## What is Absinthe?

Absinthe is a library that allows you to create actors in Rust, and communicate with them using async messages.
It provides Actor Model primitives, and a super-macro to easily create actors.
Just write your Rust code, and Absinthe will handle the rest.

## Absinthe in action

```rust
use absinthe::prelude::*;

// Actorize any async function with the #[absinthe::actor] attribute
actor! {
    async fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// It works with generics too!
// Don't think about async requirements, Absinthe will handle it for you
actor! {
    async fn sub<T>(a: T, b: T) 
    where 
        T: std::ops::Sub<Output = i32>
    {
        a - b
    }
}

// OMG ! It works with structs too!
actor! {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Self {
                count: 0,
            }
        }

        // Functions routed to the actor are marked with the #[act] attribute
        // By default, the function name is used as the message name, but you can override it
        // with the msg() attribute
        #[act(msg(inc))]
        async fn increment(&mut self) -> u32 {
            self.count += 1;
            self.count
        }

        #[act(msg(dec))]
        async fn decrement(&mut self) -> u32 {
            self.count -= 1;
            self.count
        }
    }
}

#[tokio::main]
async fn main() {
    let add_actor = add();
    let sub_actor = sub::<i32>();
    let counter = Counter::new();
    let counter = absinthe::spawn(counter);

    // send! a message to the actor, and await the response
    // notify! the actor when you don't care about the response
    let res = absinthe::send!(add_actor, 1, 2).await;
    let res = absinthe::send!(sub_actor, res, 2).await;

    // Request and Response enums are generated for each actor, based on #[act] functions
    let res = absinthe::send!(counter, CounterReq::Inc).await;
    let res = absinthe::send!(counter, CounterReq::Dec).await;

    assert_eq!(res, 1);
}

```

## Roadmap

- [x] Actor functions 
- [x] Generic Actor functions
- [ ] Lambda Actor wrapper
- [x] Actor Structs
- [ ] Actor Structs with generics
- [ ] Actor replicas
- [ ] Actor supervision
- [ ] Documentation
- [ ] Examples
- [ ] Tracing
- [ ] UDP bridge
- [ ] UDP bridge Ciphered tunnel
- [ ] UDP bridge Node Healthcheck
- [ ] RabbitMQ bridge
- [ ] Stomp bridge
