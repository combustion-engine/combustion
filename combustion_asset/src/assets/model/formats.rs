//! Model asset formats

use protocols::model::EXTENSION;

use ::asset::AssetFileFormat;
use ::assets::standard::formats::StandardFileFormat;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub enum ModelFileFormat {
    /// Native Combustion file format
    Native,
    /// Any format supported by Assimp
    #[cfg(feature = "assimp")]
    Assimp,
    /// Any standard file format
    Standard(StandardFileFormat)
}

impl AssetFileFormat for ModelFileFormat {
    #[cfg(feature = "assimp")]
    fn from_extension(ext: &str) -> Option<ModelFileFormat> {
        Some(if ext == EXTENSION {
            ModelFileFormat::Native
        } else if ::assimp::formats::is_extension_supported(ext) {
            ModelFileFormat::Assimp
        } else if let Some(standard_format) = StandardFileFormat::from_extension(ext) {
            ModelFileFormat::Standard(standard_format)
        } else {
            return None;
        })
    }

    #[cfg(not(feature = "assimp"))]
    fn from_extension(ext: &str) -> Option<ModelFileFormat> {
        Some(if ext == EXTENSION {
            ModelFileFormat::Native
        } else if let Some(standard_format) = StandardFileFormat::from_extension(ext) {
            ModelFileFormat::Standard(standard_format)
        } else {
            return None;
        })
    }

    fn can_import(&self) -> bool {
        match *self {
            ModelFileFormat::Standard(standard_format) => standard_format.can_import(),
            _ => true,
        }
    }

    fn can_export(&self) -> bool {
        match *self {
            #[cfg(feature = "assimp")]
            ModelFileFormat::Assimp => false,
            ModelFileFormat::Standard(standard_format) => standard_format.can_export(),
            _ => true,
        }
    }
}