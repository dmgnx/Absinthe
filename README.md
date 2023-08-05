# Absinthe

*Are you an Actor & Async junkie? Then your app needs some Absinthe!*

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

#[tokio::main]
async fn main() {
    let add_actor = add();
    let sub_actor = sub::<i32>();

    // send! a message to the actor, and await the response
    // notify! the actor when you don't care about the response
    let res = absinthe::send!(add_actor, 1, 2).await;
    let res = absinthe::send!(sub_actor, res, 2).await;

    assert_eq!(res, 1);
}

```

## Roadmap

- [x] Actor functions 
- [x] Generic Actor functions
- [ ] Lambda Actor wrapper
- [ ] Actor functions with context
- [ ] Actor Structs
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
