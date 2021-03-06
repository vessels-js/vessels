use std::{
    ffi::CString,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    time::{Duration, SystemTime},
};

use crate::{channel::Channel, kind, kind::Future, ConstructResult, DeconstructResult, Kind};

use futures::{SinkExt, StreamExt};

use super::WrappedError;

use void::Void;

macro_rules! primitive_impl {
    ($($ty:ident),+) => {$(
        #[kind]
        impl Kind for $ty {
            type ConstructItem = $ty;
            type ConstructError = WrappedError<Void>;
            type ConstructFuture = Future<ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructError = WrappedError<Void>;
            type DeconstructFuture = Future<DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    Ok(channel.send(self).await.map_err(WrappedError::Send)?)
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture
            {
                Box::pin(async move {
                    Ok(channel.next().await.ok_or(WrappedError::Insufficient {
                        got: 0,
                        expected: 1
                    })?)
                })
            }
        }
    )+};
}

primitive_impl!(
    bool,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
    char,
    CString,
    String,
    Ipv4Addr,
    SocketAddrV4,
    SocketAddrV6,
    SocketAddr,
    SystemTime,
    Ipv6Addr,
    Duration,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroUsize,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroIsize
);
