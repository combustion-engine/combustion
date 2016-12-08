use capnp::serialize_packed;

pub mod protocol {
    include!(concat!(env!("OUT_DIR"), "/protocols/texture_capnp.rs"));
}
