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
#[absinthe::actor]
async fn add(a: i32, b: i32) -> i32 {
    a + b
}


// It works with generics too!
// Don't think about async requirements, Absinthe will handle it for you
#[absinthe::actor]
async fn sub<T>(a: T, b: T) 
where 
    T: std::ops::Sub<Output = i32>
{
    a - b
}

// You can also create actors with a state, accessible from `self`
#[absinthe::actor]
async fn counter(&mut self, inc: u32) -> u32 {
    struct State {
        count: u32
    }

    self.count += inc;
    self.count
}

#[tokio::main]
async fn main() {
    // Every story starts with a supervisor
    let supv = Supervisor::new();

    let add = supv.spawn::<AddActor>("add");
    let sub = supv.spawn::<SubActor<u128>>("sub");
    let counter = supv.spawn::<CounterActor>("counter");

    // Send messages to actors
    assert_eq!(send!(add, 1, 2).await, 3);
    assert_eq!(send!(sub, 2u128, 1u128).await, 1u128);
    assert_eq!(send!(counter, 1).await, 1);
    assert_eq!(send!(counter, 1).await, 2);
    assert_eq!(send!(counter, 1).await, 3);

    // You can actorize closures too! (But it's blocking code until async closures are stable)
    let add = ActorClosure::new(|msg: (i32, i32)| { 
        let (a, b) = msg;
        a + b
    });
    let add = supv.deploy(add, "add");
    assert_eq!(send!(add, 1, 2).await, 3);
}

```

## Roadmap

- [x] Actor functions 
- [x] Generic Actor functions
- [x] Closure Actor wrapper
- [x] Tracing
- [~] Supervisor
- [ ] Starter (Supervising Actor)
- [ ] Actor replicas
- [ ] UDP bridge
- [ ] UDP bridge Ciphered tunnel
- [ ] UDP bridge Node Healthcheck
- [ ] RabbitMQ bridge
- [ ] Stomp bridge
