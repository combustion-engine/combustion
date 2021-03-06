//! Load and Save routines for standard (de)serializable assets

use std::io::prelude::*;

use serde::{Serialize, Deserialize};

use ::error::{AssetResult, AssetError};
use ::asset::Asset;

use ::assets::standard::formats::StandardFileFormat;

/// Load any `T: Asset` from a standard deserializable format
#[cfg_attr(not(feature = "bincode"), allow(unused_mut))]
#[cfg_attr(not(any(feature = "json", feature = "yaml", feature = "bincode")), allow(unused_variables, unreachable_code))]
pub fn load_standard_format<'a, T: 'a, R>(mut reader: R, format: StandardFileFormat) -> AssetResult<T>
    where R: Read, T: Asset<'a> + Deserialize
{
    let asset = match format {
        #[cfg(feature = "bincode")]
        StandardFileFormat::Bincode => {
            use bincode::{deserialize_from, SizeLimit};

            try_throw!(deserialize_from(&mut reader, SizeLimit::Infinite))
        },
        #[cfg(feature = "json")]
        StandardFileFormat::Json => {
            use json::from_reader;

            try_throw!(from_reader(reader))
        },
        #[cfg(feature = "yaml")]
        StandardFileFormat::Yaml => {
            use yaml::from_reader;

            try_throw!(from_reader(reader))
        },
        _ => throw!(AssetError::UnsupportedFormat),
    };

    Ok(asset)
}

/// Save any `T: Asset` to a standard serializable format
#[cfg_attr(not(feature = "json"), allow(unused_variables))]
#[cfg_attr(not(any(feature = "json", feature = "yaml", feature = "bincode")), allow(unused_mut, unreachable_code))]
pub fn save_standard_format<'a, T: 'a, W>(mut writer: W, format: StandardFileFormat, asset: &T, pretty: bool) -> AssetResult<()>
    where W: Write, T: Asset<'a> + Serialize
{
    match format {
        #[cfg(feature = "bincode")]
        StandardFileFormat::Bincode => {
            use bincode::{serialize_into, SizeLimit};

            try_throw!(serialize_into(&mut writer, asset, SizeLimit::Infinite));
        },
        #[cfg(feature = "json")]
        StandardFileFormat::Json => {
            use json::{to_writer, to_writer_pretty};

            if pretty {
                try_throw!(to_writer_pretty(&mut writer, asset));
            } else {
                try_throw!(to_writer(&mut writer, asset));
            }
        },
        #[cfg(feature = "yaml")]
        StandardFileFormat::Yaml => {
            use yaml::to_writer;

            try_throw!(to_writer(&mut writer, asset));
        },
        _ => throw!(AssetError::UnsupportedFormat),
    }

    Ok(())
}