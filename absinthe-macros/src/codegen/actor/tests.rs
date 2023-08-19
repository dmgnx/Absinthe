use crate::codegen::*;

#[test]
fn test_actor_fn() {
    let attr = quote! {};
    let input = quote! {
        async fn add(a: u32, b: u32) -> u32 {
            a + b
        }
    };

    let expected = quote! {
        struct AddActor { }

        impl AddActor {
            async fn add(a: u32, b: u32) -> u32 {
                a + b
            }
        }

        #[absinthe::prelude::async_trait]
        impl absinthe::Actor for AddActor {
            type Request = (u32, u32);
            type Response = u32;

            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::add(msg.0, msg.1).await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self { })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    );
}



#[test]
fn test_actor_fn_with_name() {
    let attr = quote! {
        name = MyActor
    };
    let input = quote! {
        async fn add(a: u32, b: u32) -> u32 {
            a + b
        }
    };

    let expected = quote! {
        struct MyActor { }

        impl MyActor {
            async fn add(a: u32, b: u32) -> u32 {
                a + b
            }
        }

        #[absinthe::prelude::async_trait]
        impl absinthe::Actor for MyActor {
            type Request = (u32, u32);
            type Response = u32;

            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::add(msg.0, msg.1).await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self { })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    );
}

#[test]
fn test_actor_fn_generic_add_function() {
    let attr = quote! {};
    let input = quote! {
        async fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }
    };

    let expected = quote! {
        struct AddActor<T : std::ops::Add<Output = T> >
        where
            T : Send + Sync + 'static
        {
            _phantom_gen: std::marker::PhantomData<T> ,
        }

        impl<T: std::ops::Add<Output = T> > AddActor<T> 
        where 
            T : Send + Sync + 'static
        {
            async fn add(a: T, b: T) -> T {
                a + b
            }
        }

        #[absinthe::prelude::async_trait]
        impl<T: std::ops::Add<Output = T> > absinthe::Actor for AddActor<T> 
        where 
            T : Send + Sync + 'static
        {
            type Request = (T, T);
            type Response = T;

            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::add(msg.0, msg.1).await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self {
                    _phantom_gen: std::marker::PhantomData ,
                })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    )
}

#[test]
fn test_actor_fn_generic_swap_function() {
    let attr = quote! {};
    let input = quote! {
        async fn swap<T,U>(a: T, b: U) -> (U, T) {
            (b, a)
        }
    };

    let expected = quote! {
        struct SwapActor<T, U> 
        where 
            T : Send + Sync + 'static,
            U : Send + Sync + 'static
        {
            _phantom_gen: std::marker::PhantomData<T, U> ,
        }

        impl<T, U> SwapActor<T, U> 
        where 
            T : Send + Sync + 'static,
            U : Send + Sync + 'static
        {
            async fn swap(a: T, b: U) -> (U, T) {
                (b, a)
            }
        }

        #[absinthe::prelude::async_trait]
        impl<T, U> absinthe::Actor for SwapActor<T, U> 
        where 
            T : Send + Sync + 'static,
            U : Send + Sync + 'static
        {
            type Request = (T, U);
            type Response = (U, T);

            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::swap(msg.0, msg.1).await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self {
                    _phantom_gen: std::marker::PhantomData ,
                })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    )
}

#[test]
fn test_actor_fn_do_nothing() {
    let attr = quote! {};
    let input = quote! {
        async fn do_nothing() -> () {
            ()
        }
    };

    let expected = quote! {
        struct DoNothingActor { }

        impl DoNothingActor {
            async fn do_nothing() -> () {
                ()
            }
        }

        #[absinthe::prelude::async_trait]
        impl absinthe::Actor for DoNothingActor {
            type Request = ();
            type Response = ();
            
            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::do_nothing().await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self { })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    )
}

#[test]
fn test_actor_fn_counter_with_state() {
    let attr = quote! {};
    let input = quote! {
        async fn counter(&mut self, inc: u32) -> u32 {
            struct State {
                #[init(0)]
                count: u32,
            }

            self.count += inc
        }
    };

    let expected = quote! {
        struct CounterActor {
            count: u32,
        }

        impl CounterActor {
            async fn counter(&mut self, inc: u32) -> u32 {
                self.count += inc
            }
        }

        #[absinthe::prelude::async_trait]
        impl absinthe::Actor for CounterActor {
            type Request = u32;
            type Response = u32;

            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::counter(self, msg).await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self {
                    count: 0,
                })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    )
}

#[test]
fn test_actor_fn_generic_counter_with_state() {
    let attr = quote! {};
    let input = quote! {
        async fn counter<T: std::ops::Add<Output = T> + Default>(&mut self, inc: T) -> T {
            struct State {
                #[init(T::default())]
                count: T
            }

            self.count += inc
        }
    };

    let expected = quote! {
        struct CounterActor<T : std::ops::Add<Output = T> + Default> 
        where
            T : Send + Sync + 'static
        {
            count: T,
            _phantom_gen: std::marker::PhantomData<T>,
        }

        impl<T : std::ops::Add<Output = T> + Default> CounterActor <T>
        where
            T : Send + Sync + 'static
        {
            async fn counter(&mut self, inc: T) -> T {
                self.count += inc
            }
        }

        #[absinthe::prelude::async_trait]
        impl<T : std::ops::Add<Output = T> + Default> absinthe::Actor for CounterActor<T> 
        where
            T : Send + Sync + 'static
        {
            type Request = T;
            type Response = T;

            async fn receive(&mut self, msg: Self::Request) -> Self::Response {
                Self::counter(self, msg).await
            }

            async fn start() -> Result<Self, absinthe::ActorError> {
                Ok(Self {
                    count: T::default(),
                    _phantom_gen: std::marker::PhantomData,
                })
            }
        }
    };

    assert_eq!(
        expected.to_string(),
        CodeGen::codegen::<ActorCodeGen>(Some(attr.into()), input.into()).to_string()
    )
}
