//! File formats for standard (de)serializable formats

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
    // Avoids errors when no standard formats are enabled
    #[doc(hidden)]
    __Invalid,
}

impl AssetFileFormat for StandardFileFormat {
    #[cfg(any(feature = "bincode", feature = "json", feature = "yaml"))]
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

    // Simple version for when all standard formats are disabled
    #[cfg(not(any(feature = "bincode", feature = "json", feature = "yaml")))]
    #[inline(always)]
    fn from_extension(_: &str) -> Option<StandardFileFormat> { None }

    #[inline(always)]
    fn can_import(&self) -> bool {
        *self != StandardFileFormat::__Invalid
    }

    #[inline(always)]
    fn can_export(&self) -> bool {
        *self != StandardFileFormat::__Invalid
    }
}