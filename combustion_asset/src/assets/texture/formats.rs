//! Texture asset formats

use image::ImageFormat;

use protocols::texture::EXTENSION;

use ::asset::AssetFileFormat;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureFileFormat {
    /// Native Combustion file format
    Native,
    /// Images that can be used with the `image` library
    Image(ImageFormat),
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

impl AssetFileFormat for TextureFileFormat {
    fn from_extension(ext: &str) -> Option<TextureFileFormat> {
        Some(if ext == EXTENSION { TextureFileFormat::Native } else {
            match ext {
                #[cfg(feature = "bincode")]
                "bc" | "bincode" => TextureFileFormat::Bincode,
                #[cfg(feature = "json")]
                "json" => TextureFileFormat::Json,
                #[cfg(feature = "yaml")]
                "yaml" => TextureFileFormat::Yaml,
                #[cfg(feature = "cbor")]
                "cbor" => TextureFileFormat::Cbor,
                _ => TextureFileFormat::Image({
                    match ext {
                        "jpg" | "jpeg" => ImageFormat::JPEG,
                        "png" => ImageFormat::PNG,
                        "gif" => ImageFormat::GIF,
                        "webp" => ImageFormat::WEBP,
                        "tif" | "tiff" => ImageFormat::TIFF,
                        "tga" => ImageFormat::TGA,
                        "ppm" => ImageFormat::PPM,
                        "bmp" => ImageFormat::BMP,
                        "ico" => ImageFormat::ICO,
                        "hdr" => ImageFormat::HDR,
                        _ => { return None; }
                    }
                })
            }
        })
    }

    fn can_import(&self) -> bool {
        match *self {
            TextureFileFormat::Image(format) => {
                match format {
                    ImageFormat::PNG |
                    ImageFormat::GIF |
                    ImageFormat::JPEG |
                    ImageFormat::WEBP |
                    ImageFormat::TIFF |
                    ImageFormat::TGA |
                    ImageFormat::BMP |
                    ImageFormat::ICO |
                    ImageFormat::HDR => true,
                    _ => false,
                }
            },
            _ => true,
        }
    }

    fn can_export(&self) -> bool {
        match *self {
            TextureFileFormat::Image(format) => {
                match format {
                    ImageFormat::ICO |
                    ImageFormat::JPEG |
                    ImageFormat::PNG |
                    ImageFormat::PPM => true,
                    _ => false,
                }
            },
            _ => true,
        }
    }
}