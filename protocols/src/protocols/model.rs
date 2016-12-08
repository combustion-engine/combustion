use capnp::serialize_packed;

pub mod protocol {
    include!(concat!(env!("OUT_DIR"), "/protocols/model_capnp.rs"));
}