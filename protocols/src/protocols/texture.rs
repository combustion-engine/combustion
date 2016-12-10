pub const EXTENSION: &'static str = "ctex";

pub mod protocol {
    include!(concat!(env!("OUT_DIR"), "/protocols/texture_capnp.rs"));
}
