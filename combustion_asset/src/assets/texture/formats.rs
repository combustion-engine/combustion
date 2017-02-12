//! Texture asset formats

use image::{ImageFormat, ImageError};

use ::error::{AssetResult, AssetError};

/// Find an appropriate image format based on file extension
pub fn image_format_from_extension(ext: &str) -> AssetResult<ImageFormat> {
    Ok(match ext {
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
        format => throw!(ImageError::UnsupportedError(format!("Image format image/{:?} is not supported.", format)))
    })
}
