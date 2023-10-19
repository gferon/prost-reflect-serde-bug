use std::sync::OnceLock;

use prost_reflect::{DescriptorPool, ReflectMessage};
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/_.rs"));

fn get_descriptor_pool() -> &'static DescriptorPool {
    static POOL: OnceLock<DescriptorPool> = OnceLock::new();
    POOL.get_or_init(|| {
        DescriptorPool::decode(
            include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
        )
        .unwrap()
    })
}

#[test]
fn test_serialization_deserialization() {
    let foo = Foo {
        bar: Some(foo::Bar::AllBars(true)),
    };
    let msg = foo.transcode_to_dynamic();
    let options = prost_reflect::SerializeOptions::new().use_proto_field_name(true);
    let mut serializer = serde_json::Serializer::new(vec![]);
    msg.serialize_with_options(&mut serializer, &options)
        .unwrap();
    let s = String::from_utf8(serializer.into_inner()).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(dbg!(&s));
    let foo2 = Foo::deserialize(&mut deserializer).unwrap();
    assert_eq!(foo, foo2);
}
