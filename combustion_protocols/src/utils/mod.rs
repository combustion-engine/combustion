//! Utility structures used within the protocols

use capnp::traits::Owned;

use ::error::ProtocolResult;

pub mod protocol {
    include!(concat!(env!("OUT_DIR"), "/protocols/utils_capnp.rs"));
}

/// Converts the option protocol into a Rust option
pub fn option_to_option<T>(option_reader: protocol::option::Reader<T>) -> ProtocolResult<Option<< T as Owned >::Reader>> where T: for<'c> Owned<'c> {
    Ok(match try_throw!(option_reader.which()) {
        protocol::option::Which::Some(value) => {
            Some(try_throw!(value))
        },
        _ => None,
    })
}