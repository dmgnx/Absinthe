use absinthe::prelude::*;

#[tokio::test]
async fn basic_struct_counter() {

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
    
            #[act(msg(inc))]
            async fn increment(&mut self) -> u32 {
                self.count += 1;
                self.count
            }
    
            #[act(msg(add))]
            async fn add(&mut self, n: u32) -> u32 {
                self.count += n;
                self.count
            }
    
            #[act(msg(dec))]
            async fn decrement(&mut self) -> u32 {
                self.count -= 1;
                self.count
            }
    
            #[act(msg(sub))]
            async fn sub(&mut self, n: u32) -> u32 {
                self.count -= n;
                self.count
            }
        }
    }

    let counter = Counter::new();
    let counter = absinthe::spawn(counter);

    assert_eq!(absinthe::send!(counter, CounterReq::Inc).await, CounterResp::Inc(1));
    assert_eq!(absinthe::send!(counter, CounterReq::Inc).await, CounterResp::Inc(2));
    assert_eq!(absinthe::send!(counter, CounterReq::Inc).await, CounterResp::Inc(3));
    assert_eq!(absinthe::send!(counter, CounterReq::Add(5)).await, CounterResp::Add(8));
    assert_eq!(absinthe::send!(counter, CounterReq::Sub(3)).await, CounterResp::Sub(5));
    assert_eq!(absinthe::send!(counter, CounterReq::Dec).await, CounterResp::Dec(4));
    assert_eq!(absinthe::send!(counter, CounterReq::Dec).await, CounterResp::Dec(3));
}