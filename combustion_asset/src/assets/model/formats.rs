//! Model asset formats

use assimp;

use protocols::model::EXTENSION;

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

impl ModelFileFormat {
    /// Gets a `ModelFileFormat` from the given file extension if possible.
    ///
    /// `None` is returned if no format exists for that file extension.
    pub fn from_extension(ext: &str) -> Option<ModelFileFormat> {
        Some({
            if ext == EXTENSION {
                ModelFileFormat::Native
            } else {
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
                    _ => return None,
                }
            }
        })
    }

    /// Checks if a certain format can be imported
    pub fn can_import(&self) -> bool {
        true
    }

    /// Checks if a certain format can be exported
    pub fn can_export(&self) -> bool {
        match *self {
            ModelFileFormat::Assimp => false,
            _ => true,
        }
    }
}