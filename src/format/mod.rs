#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json")]
#[doc(inline)]
pub use json::Json;
pub mod cbor;
#[doc(inline)]
pub use cbor::Cbor;
#[cfg(feature = "bincode")]
pub mod bincode;
#[cfg(feature = "bincode")]
#[doc(inline)]
pub use bincode::Bincode;

use predicated_ordered::BufferedPredicatedExt;

use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    future::ok,
    Future as IFuture, FutureExt, Sink as ISink, SinkExt, Stream as IStream, StreamExt,
    TryFutureExt,
};

use crate::{
    channel::{Context, Shim, Target, Waiter},
    core::spawn,
    kind::{Fallible, SinkStream},
    ErrorBound, Kind,
};

use serde::{de::DeserializeSeed, Serialize};

use core::fmt::{self, Debug, Formatter};

use thiserror::Error;

pub trait UniformStreamSink<T>: ISink<T> + IStream<Item = T> {}

impl<T, U> UniformStreamSink<T> for U where U: ISink<T> + IStream<Item = T> {}

/// A serialization format used in the transport of `Kind`s.
///
/// This is generally a minimal wrapper that encapsulates a `serde` format.
pub trait Format {
    /// The underlying representation used by this `Format`, i.e. `Vec<u8>` for most
    /// binary formats and `String` for those of a human-readable nature.
    type Representation;
    /// The failure condition of this format. This may be encountered during deserialization.
    type Error: ErrorBound;

    /// Serializes the provided item.
    fn serialize<T: Serialize>(item: T) -> Self::Representation
    where
        Self: Sized;
    /// Deserializes an item from the provided formatted representation.
    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> Fallible<T::Value, (Self::Error, Self::Representation)>
    where
        T: Sync + Send + 'static,
        Self: Sized;
}

pub trait ApplyEncode<'de>:
    Sized + UniformStreamSink<<Self as Context<'de>>::Item> + Context<'de>
where
    <Self as ISink<<Self as Context<'de>>::Item>>::Error: ErrorBound,
{
    fn encode<F: Format + Encode<'de, Self>>(self) -> <F as Encode<'de, Self>>::Output;
}

impl<'de, T> ApplyEncode<'de> for T
where
    T: UniformStreamSink<<Self as Context<'de>>::Item> + Context<'de>,
    <T as ISink<<Self as Context<'de>>::Item>>::Error: ErrorBound,
{
    fn encode<F: Format + Encode<'de, Self>>(self) -> <F as Encode<'de, Self>>::Output {
        <F as Encode<_>>::encode(self)
    }
}

pub trait ApplyDecode<'de, K: Kind> {
    fn decode<T: Target<'de, K> + Sync + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, K>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Sync + Send + Sized + 'static,
        F::Representation: Clone + Sync + Send + 'static,
        <Self as ISink<F::Representation>>::Error: ErrorBound,
        T::Item: Sync + Send + 'static;
}

impl<'de, U, K: Kind> ApplyDecode<'de, K> for U {
    fn decode<T: Target<'de, K> + Sync + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, K>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Sync + Send + Sized + 'static,
        F::Representation: Clone + Sync + Send,
        <Self as ISink<F::Representation>>::Error: ErrorBound,
        T::Item: Sync + Send,
    {
        <F as Decode<'de, Self, K>>::decode::<T>(self)
    }
}

pub trait Decode<'de, C: UniformStreamSink<<Self as Format>::Representation> + 'static, K: Kind>:
    Format
where
    C::Error: 'static,
{
    type Output: IFuture<Output = Result<K, K::ConstructError>>;

    fn decode<T: Target<'de, K> + Sync + Send + 'static>(input: C) -> Self::Output
    where
        T::Item: Sync + Send;
}

pub trait Encode<'de, C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de>>:
    Format + Sized
where
    <C as ISink<<C as Context<'de>>::Item>>::Error: ErrorBound,
{
    type Output: IStream<Item = <Self as Format>::Representation>
        + ISink<Self::Representation, Error = EncodeError<Self, <C as Context<'de>>::Item, C>>;

    fn encode(input: C) -> Self::Output;
}

impl<
        'de,
        C: Sync + Send + UniformStreamSink<<Self as Format>::Representation> + 'static,
        T: Format + 'static,
        K: Kind,
    > Decode<'de, C, K> for T
where
    Self::Representation: Sync + Send + Clone,
    <C as ISink<<Self as Format>::Representation>>::Error: ErrorBound,
{
    type Output = Fallible<K, K::ConstructError>;

    fn decode<U: Target<'de, K> + Sync + Send + 'static>(input: C) -> Self::Output
    where
        U::Item: Sync + Send,
    {
        let shim = U::new_shim();
        let context = shim.context();
        let ctx = context.clone();
        let (sink, stream) = input.split();
        Box::pin(
            shim.complete(SinkStream::new(
                sink.sink_map_err(|_| panic!())
                    .with::<_, _, _, ()>(|item: U::Item| ok(Self::serialize(item))),
                stream
                    .map(move |item| {
                        let ct = context.clone();
                        Self::deserialize(item, context.clone())
                            .or_else(move |(e, item)| {
                                let context = ct.clone();
                                let message = format!("{}", e);
                                let mut data = message.split_whitespace();
                                if data.next() == Some("ASYNC_WAIT") {
                                    if let Some(data) = data.next() {
                                        return context.wait_for(data.to_owned()).then(move |_| {
                                            Self::deserialize(item, context.clone())
                                        });
                                    }
                                }
                                panic!(format!("{:?}", e))
                            })
                            .unwrap_or_else(|e| panic!(format!("{:?}", e.0)))
                    })
                    .buffered_predicated(core::usize::MAX, move |i| ctx.predicate(i)),
            )),
        )
    }
}

#[derive(Error)]
pub enum EncodeError<T: Format, I, S: ISink<I>>
where
    S::Error: ErrorBound,
{
    #[error("{0}")]
    Format(#[source] T::Error),
    #[error("{0}")]
    Sink(#[source] S::Error),
}

impl<T: Format, I, S: ISink<I>> Debug for EncodeError<T, I, S>
where
    S::Error: ErrorBound,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EncodeError::Format(e) => format!("Format ({:?})", e),
                EncodeError::Sink(e) => format!("Sink ({:?})", e),
            }
        )
    }
}

impl<T: Format, I, S: ISink<I>> EncodeError<T, I, S>
where
    S::Error: ErrorBound,
{
    fn from_sink_error(err: S::Error) -> Self {
        EncodeError::Sink(err)
    }
    fn from_format_error(err: T::Error) -> Self {
        EncodeError::Format(err)
    }
}

impl<
        'de,
        T: Format + 'static,
        C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de> + 'static + Sync + Send + Sized,
    > Encode<'de, C> for T
where
    T::Representation: Sync + Send + Clone,
    <C as Context<'de>>::Item: Sync + Send,
    <C as ISink<<C as Context<'de>>::Item>>::Error: ErrorBound,
{
    type Output = SinkStream<
        Self::Representation,
        EncodeError<T, <C as Context<'de>>::Item, C>,
        Self::Representation,
    >;

    fn encode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        let (sender, receiver): (_, UnboundedReceiver<<Self as Format>::Representation>) =
            unbounded();
        let predicate_ctx = ctx.clone();
        let receiver = receiver
            .map(move |item: <Self as Format>::Representation| {
                let ct = ctx.clone();
                Self::deserialize(item, ctx.clone()).or_else(move |(e, item)| {
                    let context = ct.clone();
                    let message = format!("{}", e);
                    let mut data = message.split_whitespace();
                    if data.next() == Some("ASYNC_WAIT") {
                        if let Some(data) = data.next() {
                            return context
                                .wait_for(data.to_owned())
                                .then(move |_| Self::deserialize(item, context.clone()));
                        }
                    }
                    panic!(format!("{:?}", e))
                })
            })
            .buffered_predicated(core::usize::MAX, move |i| {
                predicate_ctx.predicate(i.as_ref().unwrap_or_else(|_| panic!()))
            });
        spawn(
            receiver
                .forward(sink.sink_map_err(|e| panic!(format!("{}", e))))
                .unwrap_or_else(|_| panic!()),
        );
        SinkStream::new(
            sender.sink_map_err(|e| panic!(format!("{}", e))),
            stream.map(<Self as Format>::serialize),
        )
    }
}
