use vessels::{
    channel::IdChannel,
    core::run,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::using,
    log, Kind, OnTo,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Kind, Debug)]
#[kind(using::Serde)]
pub struct WithSerde {
    test: u32,
}

#[derive(Kind, Debug)]
enum Enum<T: Kind> {
    Unit,
    QualifiedInline(#[kind(using::Iterator)] Vec<u32>),
    Tuple(String, u32),
    StructLike { item: T, another_kind: WithSerde },
}

fn main() {
    run(async move {
        let encoded = Enum::StructLike {
            item: "hello".to_owned(),
            another_kind: WithSerde { test: 10 },
        }
        .on_to::<IdChannel>()
        .await
        .encode::<Cbor>();
        let decoded: Enum<String> = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{:?}", decoded);
    });
}
