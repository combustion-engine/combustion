//! Texture asset formats

use image::ImageFormat;

use protocols::texture::EXTENSION;

use ::asset::AssetFileFormat;

use ::assets::standard::formats::StandardFileFormat;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureFileFormat {
    /// Native Combustion file format
    Native,
    /// Images that can be used with the `image` library
    Image(ImageFormat),
    /// Any standard file format
    StandardFormat(StandardFileFormat),
}

impl AssetFileFormat for ImageFormat {
    fn from_extension(ext: &str) -> Option<ImageFormat> {
        Some(match ext {
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
        })
    }

    fn can_import(&self) -> bool {
        match *self {
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
    }

    fn can_export(&self) -> bool {
        match *self {
            ImageFormat::ICO |
            ImageFormat::JPEG |
            ImageFormat::PNG |
            ImageFormat::PPM => true,
            _ => false,
        }
    }
}

impl AssetFileFormat for TextureFileFormat {
    fn from_extension(ext: &str) -> Option<TextureFileFormat> {
        Some(if ext == EXTENSION {
            TextureFileFormat::Native
        } else if let Some(image_format) = ImageFormat::from_extension(ext) {
            TextureFileFormat::Image(image_format)
        } else if let Some(standard_format) = StandardFileFormat::from_extension(ext) {
            TextureFileFormat::StandardFormat(standard_format)
        } else {
            return None;
        })
    }

    fn can_import(&self) -> bool {
        match *self {
            TextureFileFormat::Image(image_format) => image_format.can_import(),
            TextureFileFormat::StandardFormat(standard_format) => standard_format.can_import(),
            _ => true,
        }
    }

    fn can_export(&self) -> bool {
        match *self {
            TextureFileFormat::Image(image_format) => image_format.can_export(),
            TextureFileFormat::StandardFormat(standard_format) => standard_format.can_export(),
            _ => true,
        }
    }
}