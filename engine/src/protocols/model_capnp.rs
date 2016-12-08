extern crate capnp;

pub mod protocol {
    include!(concat!(env!("OUT_DIR"), "/protocols/model_capnp.rs"));
}

pub mod model {
    use super::protocol::{mesh, model, option};
    use capnp::serialize_packed;
}