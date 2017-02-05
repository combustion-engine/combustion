//! Texture Asset

use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use image::{self, DynamicImage, GenericImage, ImageFormat};

use protocols::traits::Storage;
use protocols::texture::{protocol, EXTENSION};
use protocols::texture::data::{texture, format};
use protocols::texture::storage::RootTextureQuery;

use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery};

/// Texture Asset
pub struct TextureAsset(texture::RootTexture);

/// Save/Load arguments for texture assets
#[derive(Debug, Clone, Copy)]
pub struct TextureArgs {
    /// Only allow 2D textures
    pub only2d: bool,
    /// Consider the loaded images as in sRGB color space
    pub srgb: bool,
}

impl Default for TextureArgs {
    fn default() -> TextureArgs {
        TextureArgs { only2d: false, srgb: false }
    }
}

/// Texture asset query
#[derive(Debug, Clone, Copy)]
pub enum TextureAssetQuery<'a> {
    /// Queries if a given medium is supported
    SupportedMedium(AssetMedium<'a>)
}

impl<'a> AssetQuery for TextureAssetQuery<'a> {
    type Arguments = TextureAssetQuery<'a>;
    type Result = bool;
}

/// Find an appropriate image format based on file extension
pub fn image_format_from_extension(ext: &str) -> AssetResult<ImageFormat> {
    Ok(match ext {
        "jpg" | "jpeg" => ImageFormat::JPEG,
        "png" => ImageFormat::PNG,
        "gif" => ImageFormat::GIF,
        "webp" => ImageFormat::WEBP,
        "tif" | "tiff" => ImageFormat::TIFF,
        "tga" => ImageFormat::TGA,
        "bmp" => ImageFormat::BMP,
        "ico" => ImageFormat::ICO,
        "hdr" => ImageFormat::HDR,
        format => throw!(image::ImageError::UnsupportedError(format!("Image format image/{:?} is not supported.", format)))
    })
}

impl<'a> Asset<'a> for TextureAsset {
    type LoadArgs = TextureArgs;
    type SaveArgs = TextureArgs;

    type Query = TextureAssetQuery<'a>;

    fn query(query: TextureAssetQuery) -> AssetResult<bool> {
        Ok(match query {
            TextureAssetQuery::SupportedMedium(medium) => {
                match medium {
                    AssetMedium::Memory => false,
                    AssetMedium::File(path) => {
                        //TODO
                        true
                    }
                }
            }
        })
    }

    fn load<R: BufRead + Seek>(mut reader: R, medium: AssetMedium<'a>, args: TextureArgs) -> AssetResult<TextureAsset> {
        if let AssetMedium::File(path) = medium {
            if let Some(ext) = path.extension() {
                let ext = ext.to_str().unwrap().to_ascii_lowercase();

                if ext == EXTENSION {
                    let message_reader = try_throw!(serialize_packed::read_message(&mut reader, ReaderOptions {
                        traversal_limit_in_words: u64::max_value(),
                        nesting_limit: 64,
                    }));

                    let root_texture_reader = try_throw!(message_reader.get_root::<protocol::root_texture::Reader>());

                    let query_results = try_rethrow!(texture::RootTexture::query_reader(root_texture_reader.borrow()));

                    if args.only2d && query_results != RootTextureQuery::Single {
                        throw!(AssetError::InvalidValue);
                    }

                    let root_texture = try_rethrow!(texture::RootTexture::load_from_reader(root_texture_reader));

                    return Ok(TextureAsset(root_texture));
                } else {
                    let image_format = image_format_from_extension(ext.as_str())?;

                    // Load ordinary image into data structures
                    let image: DynamicImage = try_throw!(image::load(reader, image_format));

                    let format = format::SpecificFormat {
                        which: format::Which::None(format::Uncompressed {
                            channels: match image {
                                DynamicImage::ImageLuma8(_) => protocol::Channels::R,
                                DynamicImage::ImageLumaA8(_) => protocol::Channels::Rg,
                                DynamicImage::ImageRgb8(_) => protocol::Channels::Rgb,
                                DynamicImage::ImageRgba8(_) => protocol::Channels::Rgba,
                            },
                            data_type: protocol::DataType::UnsignedByte,
                        }),
                        srgb: args.srgb
                    };

                    let (width, height) = image.dimensions();

                    return Ok(TextureAsset(texture::RootTexture::Single(box texture::Texture {
                        data: image.raw_pixels().into(),
                        dimensions: texture::Dimensions::new(width, height, 0),
                        kind: {
                            if (width == 1 || height == 1) && !args.only2d {
                                protocol::TextureKind::Texture1D
                            } else {
                                protocol::TextureKind::Texture2D
                            }
                        },
                        format: format,
                    })));
                }
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }

    fn save<W: Write>(&self, writer: W, medium: AssetMedium<'a>, args: TextureArgs) -> AssetResult<()> {
        if let AssetMedium::File(path) = medium {
            unimplemented!()
        }

        throw!(AssetError::UnsupportedMedium)
    }
}

impl Deref for TextureAsset {
    type Target = texture::RootTexture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}