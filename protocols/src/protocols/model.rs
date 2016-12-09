use capnp::serialize_packed;

pub const EXTENSION: &'static str = "cmodel";

pub mod protocol {
    include!(concat!(env!("OUT_DIR"), "/protocols/model_capnp.rs"));
}