//! Model asset formats

use assimp;

use protocols::model::EXTENSION;

use ::asset::AssetFileFormat;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub enum ModelFileFormat {
    /// Native Combustion file format
    Native,
    /// Any format supported by Assimp
    Assimp,
    /// Bincode
    #[cfg(feature = "bincode")]
    Bincode,
    /// JSON
    #[cfg(feature = "json")] Json,
    /// YAML
    #[cfg(feature = "yaml")] Yaml,
    /// CBOR
    #[cfg(feature = "cbor")] Cbor,
}

impl AssetFileFormat for ModelFileFormat {
    fn from_extension(ext: &str) -> Option<ModelFileFormat> {
        Some(if ext == EXTENSION { ModelFileFormat::Native } else {
            match ext {
                #[cfg(feature = "bincode")]
                "bc" | "bincode" => ModelFileFormat::Bincode,
                #[cfg(feature = "json")]
                "json" => ModelFileFormat::Json,
                #[cfg(feature = "yaml")]
                "yaml" => ModelFileFormat::Yaml,
                #[cfg(feature = "cbor")]
                "cbor" => ModelFileFormat::Cbor,
                _ if assimp::formats::is_extension_supported(ext) => {
                    ModelFileFormat::Assimp
                },
                _ => { return None; },
            }
        })
    }

    fn can_import(&self) -> bool { true }

    fn can_export(&self) -> bool {
        match *self {
            ModelFileFormat::Assimp => false,
            _ => true,
        }
    }
}