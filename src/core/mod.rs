use failure::{Error, Fail};
use futures::{lock, SinkExt, StreamExt};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
};

use crate::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::{Future, SinkStream},
    object,
    replicate::Share,
    Kind, OnTo,
};

mod executor;
pub use executor::{run, spawn};

pub mod data;
pub mod hal;
pub mod orchestrator;

pub type Constructor<T> = Box<dyn FnOnce(Handle) -> Future<T> + Send + Sync>;

#[derive(Fail, Debug, Kind)]
#[fail(display = "{} is unimplemented on this target", feature)]
pub struct UnimplementedError {
    feature: String,
}

#[derive(Fail, Debug, Kind)]
pub enum CoreError {
    Unavailable,
    Unimplemented(#[fail(cause)] UnimplementedError),
    Construct(#[fail(cause)] Error),
}

impl Display for CoreError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use CoreError::{Construct, Unavailable, Unimplemented};
        write!(
            formatter,
            "{}",
            match self {
                Unavailable => "this feature is unavailable or unregistered".to_owned(),
                Unimplemented(feature) => format!("{}", feature),
                Construct(e) => format!("handle transfer failed: {}", e),
            }
        )
    }
}

#[doc(hidden)]
pub struct Logger(());

impl Logger {
    pub fn info(&self, _message: String) {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        web_sys::console::log_1(&_message.into());
        #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
        unimplemented!();
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", _message);
    }
}

lazy_static! {
    #[doc(hidden)]
    pub static ref LOG: Logger = Logger(());
}

#[cfg(all(target_arch = "wasm32", not(feature = "core")))]
lazy_static! {
    static ref HANDLE: Mutex<Option<Handle>> = Mutex::new(None);
}

#[cfg(all(target_arch = "wasm32", not(feature = "core")))]
#[doc(hidden)]
pub fn register_handle(item: Handle) {
    let mut handle = HANDLE.lock().unwrap();
    if handle.is_none() {
        *handle = Some(item)
    }
}

pub fn acquire<K: Kind>() -> Future<Result<K, CoreError>> {
    #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
    return {
        let handle = HANDLE.lock().unwrap();
        if let Some(handle) = &*handle {
            handle.acquire::<K>()
        } else {
            Box::pin(async { Err(CoreError::Unavailable) })
        }
    };
    #[cfg(not(all(target_arch = "wasm32", not(feature = "core"))))]
    Box::pin(async { Err(CoreError::Unavailable) })
}

#[object]
trait HandleInner {
    fn acquire(
        &self,
        ty: [u8; 32],
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>>;
}

#[derive(Kind)]
pub struct Handle(Box<dyn HandleInner>);

impl Handle {
    pub fn acquire<K: Kind>(&self) -> Future<Result<K, CoreError>> {
        let channel = self.0.acquire(K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD);
        Box::pin(async move {
            channel
                .await?
                .sink_map_err(CoreError::Construct)
                .decode::<IdChannel, Cbor>()
                .await
                .map_err(|e: K::ConstructError| CoreError::Construct(e.into()))
        })
    }
}

pub struct Core {
    capabilities: Arc<
        Mutex<
            HashMap<
                [u8; 32],
                Box<
                    dyn Fn() -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>>
                        + Sync
                        + Send,
                >,
            >,
        >,
    >,
}

impl HandleInner for Core {
    fn acquire(
        &self,
        ty: [u8; 32],
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>> {
        if let Some(capability) = self.capabilities.lock().unwrap().get(&ty) {
            capability()
        } else {
            Box::pin(async move { Err(CoreError::Unavailable) })
        }
    }
}

impl Core {
    pub fn new() -> Self {
        Core {
            capabilities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn register<K: Kind>(&mut self, item: impl Fn() -> K + Sync + Send + 'static) {
        let item = Arc::new(lock::Mutex::new(item));
        self.capabilities.lock().unwrap().insert(
            K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD,
            Box::new(move || {
                let item = item.clone();
                Box::pin(async move {
                    let (sink, stream) = (item.lock().await)()
                        .on_to::<IdChannel>()
                        .await
                        .encode::<Cbor>()
                        .split();
                    Ok(SinkStream::new(sink.sink_map_err(Error::from), stream))
                })
            }),
        );
    }
    pub fn into_handle(self) -> Handle {
        Handle(Box::new(self))
    }
}

impl Share for Core {
    fn share(&self) -> Self {
        Core {
            capabilities: self.capabilities.clone(),
        }
    }
}
