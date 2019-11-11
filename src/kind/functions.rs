use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{
    future::BoxFuture, lock::Mutex, stream::BoxStream, FutureExt, SinkExt, StreamExt, TryFutureExt,
};

use std::sync::Arc;

use void::Void;

impl<U: Kind> Kind for Box<dyn Fn() -> BoxFuture<'static, U> + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)().await).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Box<dyn Fn() -> BoxFuture<'static, U> + Send + Sync> =
                Box::new(move || {
                    let channel = channel.clone();
                    Box::pin(async move {
                        let mut channel = channel.lock().await;
                        channel.send(()).unwrap_or_else(|_| panic!()).await;
                        let handle = channel.next().await.expect("test2");
                        channel.get_fork(handle).await.expect("test3")
                    })
                });
            Ok(closure)
        })
    }
}

impl<U: Kind> Kind for Box<dyn Fn() -> BoxStream<'static, U> + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)()).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Box<dyn Fn() -> BoxStream<'static, U> + Send + Sync> =
                Box::new(move || {
                    let channel = channel.clone();
                    Box::pin(
                        async move {
                            let mut channel = channel.lock().await;
                            channel.send(()).unwrap_or_else(|_| panic!()).await;
                            let handle = channel.next().await.expect("test2");
                            channel
                                .get_fork::<BoxStream<'static, U>>(handle)
                                .await
                                .expect("test3")
                        }
                        .into_stream()
                        .flatten(),
                    )
                });
            Ok(closure)
        })
    }
}

impl<U: Kind> Kind for Box<dyn FnMut() -> BoxFuture<'static, U> + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        mut self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)().await).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Box<dyn FnMut() -> BoxFuture<'static, U> + Send + Sync> =
                Box::new(move || {
                    let channel = channel.clone();
                    Box::pin(async move {
                        let mut channel = channel.lock().await;
                        channel.send(()).unwrap_or_else(|_| panic!()).await;
                        let handle = channel.next().await.expect("test2");
                        channel.get_fork(handle).await.expect("test3")
                    })
                });
            Ok(closure)
        })
    }
}

impl<U: Kind> Kind for Box<dyn FnMut() -> BoxStream<'static, U> + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        mut self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)()).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Box<dyn FnMut() -> BoxStream<'static, U> + Send + Sync> =
                Box::new(move || {
                    let channel = channel.clone();
                    Box::pin(
                        async move {
                            let mut channel = channel.lock().await;
                            channel.send(()).unwrap_or_else(|_| panic!()).await;
                            let handle = channel.next().await.expect("test2");
                            channel
                                .get_fork::<BoxStream<'static, U>>(handle)
                                .await
                                .expect("test3")
                        }
                        .into_stream()
                        .flatten(),
                    )
                });
            Ok(closure)
        })
    }
}

impl<U: Kind> Kind for Box<dyn FnOnce() -> BoxFuture<'static, U> + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel.next().await.unwrap();
            channel
                .send(channel.fork((self)().await).await.unwrap())
                .unwrap_or_else(|_| panic!())
                .await;
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let closure: Box<dyn FnOnce() -> BoxFuture<'static, U> + Send + Sync> =
                Box::new(move || {
                    Box::pin(async move {
                        channel.send(()).unwrap_or_else(|_| panic!()).await;
                        let handle = channel.next().await.expect("test2");
                        channel.get_fork(handle).await.expect("test3")
                    })
                });
            Ok(closure)
        })
    }
}

impl<U: Kind> Kind for Box<dyn FnOnce() -> BoxStream<'static, U> + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel.next().await.unwrap();
            channel
                .send(channel.fork((self)()).await.unwrap())
                .unwrap_or_else(|_| panic!())
                .await;
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let closure: Box<dyn FnOnce() -> BoxStream<'static, U> + Send + Sync> =
                Box::new(move || {
                    Box::pin(
                        async move {
                            channel.send(()).unwrap_or_else(|_| panic!()).await;
                            let handle = channel.next().await.expect("test2");
                            channel
                                .get_fork::<BoxStream<'static, U>>(handle)
                                .await
                                .expect("test3")
                        }
                        .into_stream()
                        .flatten(),
                    )
                });
            Ok(closure)
        })
    }
}

macro_rules! functions_impl {
    ($($len:expr => ($($n:tt $name:ident $nn:ident)+))+) => {$(
        #[allow(non_snake_case)]
        impl<U: Kind, $($name),+> Kind for Box<dyn Fn($($name),+) -> BoxFuture<'static, U> + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handles) = channel.next().await {
                        channel
                            .send(
                                channel
                                    .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+).await)
                                    .await
                                    .unwrap(),
                            )
                            .unwrap_or_else(|_| panic!())
                            .await;
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Box<dyn Fn($($name),+) -> BoxFuture<'static, U> + Send + Sync> =
                        Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            Box::pin(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        impl<U: Kind, $($name),+> Kind for Box<dyn FnMut($($name),+) -> BoxFuture<'static, U> + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                mut self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handles) = channel.next().await {
                        channel
                            .send(
                                channel
                                    .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+).await)
                                    .await
                                    .unwrap(),
                            )
                            .unwrap_or_else(|_| panic!())
                            .await;
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Box<dyn FnMut($($name),+) -> BoxFuture<'static, U> + Send + Sync> =
                        Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            Box::pin(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        impl<U: Kind, $($name),+> Kind for Box<dyn FnMut($($name),+) -> BoxStream<'static, U> + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                mut self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handles) = channel.next().await {
                        channel
                            .send(
                                channel
                                    .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                    .await
                                    .unwrap(),
                            )
                            .unwrap_or_else(|_| panic!())
                            .await;
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Box<dyn FnMut($($name),+) -> BoxStream<'static, U> + Send + Sync> =
                        Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            Box::pin(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork::<BoxStream<'static, U>>(handle).await.expect("test3")
                            }.into_stream().flatten())
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        impl<U: Kind, $($name),+> Kind for Box<dyn Fn($($name),+) -> BoxStream<'static, U> + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handles) = channel.next().await {
                        channel
                            .send(
                                channel
                                    .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                    .await
                                    .unwrap(),
                            )
                            .unwrap_or_else(|_| panic!())
                            .await;
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Box<dyn Fn($($name),+) -> BoxStream<'static, U> + Send + Sync> =
                        Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            Box::pin(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork::<BoxStream<'static, U>>(handle).await.expect("test3")
                            }.into_stream().flatten())
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        impl<U: Kind, $($name),+> Kind for Box<dyn FnOnce($($name),+) -> BoxFuture<'static, U> + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    let handles = channel.next().await.unwrap();
                    channel
                        .send(
                            channel
                                .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+).await)
                                .await
                                .unwrap(),
                        )
                        .unwrap_or_else(|_| panic!())
                        .await;
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let closure: Box<dyn FnOnce($($name),+) -> BoxFuture<'static, U> + Send + Sync> =
                        Box::new(move |$($name),+| {
                            Box::pin(async move {
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        impl<U: Kind, $($name),+> Kind for Box<dyn FnOnce($($name),+) -> BoxStream<'static, U> + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    let handles = channel.next().await.unwrap();
                    channel
                        .send(
                            channel
                                .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                .await
                                .unwrap(),
                        )
                        .unwrap_or_else(|_| panic!())
                        .await;
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let closure: Box<dyn FnOnce($($name),+) -> BoxStream<'static, U> + Send + Sync> =
                        Box::new(move |$($name),+| {
                            Box::pin(async move {
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork::<BoxStream<'static, U>>(handle).await.expect("test3")
                            }.into_stream().flatten())
                        });
                    Ok(closure)
                })
            }
        })+
    }
}

functions_impl! {
    1 => (0 T0 a)
    2 => (0 T0 a 1 T1 b)
    3 => (0 T0 a 1 T1 b 2 T2 c)
    4 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d)
    5 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e)
    6 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f)
    7 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g)
    8 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h)
    9 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i)
    10 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j)
    11 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k)
    12 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l)
    13 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m)
    14 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n)
    15 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n 14 T14 o)
    16 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n 14 T14 o 15 T15 p)
}
