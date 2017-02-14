//! File formats for standard (de)serializable formats

// Just allow everything if no standard formats are enabled
#![cfg_attr(not(any(feature = "bincode", feature = "json", feature = "yaml")),
allow(unused_variables, dead_code, unreachable_code, unused_mut, unreachable_patterns))]

use ::asset::AssetFileFormat;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub enum StandardFileFormat {
    /// Bincode
    #[cfg(feature = "bincode")]
    Bincode,
    /// JSON
    #[cfg(feature = "json")] Json,
    /// YAML
    #[cfg(feature = "yaml")] Yaml,
    // Used when no features are enabled
    #[doc(hidden)]
    __Invalid,
}

impl AssetFileFormat for StandardFileFormat {
    fn from_extension(ext: &str) -> Option<StandardFileFormat> {
        Some(match ext {
            #[cfg(feature = "bincode")]
            "bc" | "bincode" => StandardFileFormat::Bincode,
            #[cfg(feature = "json")]
            "json" => StandardFileFormat::Json,
            #[cfg(feature = "yaml")]
            "yaml" => StandardFileFormat::Yaml,
            _ => { return None; },
        })
    }

    fn can_import(&self) -> bool {
        match *self {
            StandardFileFormat::__Invalid => false,
            _ => true,
        }
    }
    fn can_export(&self) -> bool {
        match *self {
            StandardFileFormat::__Invalid => false,
            _ => true,
        }
    }
}