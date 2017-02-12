//! Texture asset implementation

use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::io::BufReader;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use image::{self, DynamicImage, GenericImage, ImageFormat};

use protocols::traits::Storage;
use protocols::texture::{protocol, EXTENSION};
use protocols::texture::data::{texture, format};
use protocols::texture::storage::RootTextureQuery;

use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery};

use super::formats::image_format_from_extension;

/// Texture Asset
pub struct TextureAsset(texture::RootTexture);

/// Load arguments for texture assets
#[derive(Debug, Clone, Copy)]
pub struct TextureAssetLoadArgs {
    /// Only allow 2D textures
    pub only2d: bool,
    /// Consider the loaded images as in sRGB color space
    pub srgb: bool,
    /// If a filepath is given, it'll first try to use that for determining the image format.
    ///
    /// If it cannot determine the image format from the path, it will use this hint.
    ///
    /// If the hint is `None`, it'll default to the Combustion texture format.
    pub format_hint: Option<ImageFormat>,
}

impl Default for TextureAssetLoadArgs {
    fn default() -> TextureAssetLoadArgs {
        TextureAssetLoadArgs { only2d: false, srgb: false, format_hint: None }
    }
}

/// Save arguments for texture assets
#[derive(Debug, Clone, Copy)]
pub struct TextureAssetSaveArgs {
    /// If a filepath is given, it'll first try to use that for determining the image format.
    ///
    /// If it cannot determine the image format from the path, it will use this hint.
    ///
    /// If the hint is `None`, it'll default to the Combustion texture format.
    pub format_hint: Option<ImageFormat>,
    /// For formats with adjustable encoding quality,
    /// set the quality as a value between 1-100 where 1 is the worst and 100 is the best.
    pub quality: u8,
}

impl Default for TextureAssetSaveArgs {
    fn default() -> TextureAssetSaveArgs {
        TextureAssetSaveArgs {
            format_hint: None,
            quality: 95,
        }
    }
}

/// Texture asset query
#[derive(Debug, Clone)]
pub enum TextureAssetQuery<'a> {
    /// Queries if a given medium is supported
    SupportedMedium(AssetMedium<'a>)
}

impl<'a> AssetQuery for TextureAssetQuery<'a> {
    type Arguments = TextureAssetQuery<'a>;
    type Result = bool;
}

impl<'a> Asset<'a> for TextureAsset {
    type LoadArgs = TextureAssetLoadArgs;
    type SaveArgs = TextureAssetSaveArgs;

    type Query = TextureAssetQuery<'a>;

    fn query(query: TextureAssetQuery) -> AssetResult<bool> {
        Ok(match query {
            TextureAssetQuery::SupportedMedium(medium) => {
                if let AssetMedium::File(_, _) = medium { true } else { false }
            },
        })
    }

    fn load(medium: AssetMedium<'a>, args: TextureAssetLoadArgs) -> AssetResult<TextureAsset> {
        if let AssetMedium::File(path, vfs) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                let mut reader = BufReader::new(try_throw!(vfs.open(path)));

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
                    let image: DynamicImage = try_throw!(image::load(&mut reader, image_format));

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

    fn save(&self, medium: AssetMedium<'a>, args: TextureAssetSaveArgs) -> AssetResult<()> {
        if let AssetMedium::File(path, vfs) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                let mut writer = try_throw!(vfs.create_or_truncate(path));

                if ext == EXTENSION {
                    let mut message = ::capnp::message::Builder::new_default();

                    {
                        let root_texture_builder = message.init_root::<protocol::root_texture::Builder>();

                        try_rethrow!(self.0.save_to_builder(root_texture_builder));
                    }

                    try_throw!(serialize_packed::write_message(&mut writer, &message));

                    return Ok(());
                } else if let texture::RootTexture::Single(ref texture) = **self {
                    let image_format = image_format_from_extension(ext.as_str())?;

                    if !texture.is_compressed() {
                        if texture.kind == protocol::TextureKind::Texture2D ||
                            texture.kind == protocol::TextureKind::Texture1D {
                            if let Some(bit_depth) = texture.format.which.data_type().bit_depth() {
                                let color_type = match texture.format.which.channels() {
                                    protocol::Channels::R => image::ColorType::Gray(bit_depth),
                                    protocol::Channels::Rg => image::ColorType::GrayA(bit_depth),
                                    protocol::Channels::Rgb => image::ColorType::RGB(bit_depth),
                                    protocol::Channels::Rgba => image::ColorType::RGBA(bit_depth),
                                };

                                let (width, height, _) = texture.dimensions.to_tuple();

                                let result = match image_format {
                                    ImageFormat::ICO => {
                                        image::ico::ICOEncoder::new(writer)
                                            .encode(texture.data.as_slice(), width, height, color_type)
                                    },
                                    ImageFormat::JPEG => {
                                        image::jpeg::JPEGEncoder::new_with_quality(&mut writer, args.quality)
                                            .encode(texture.data.as_slice(), width, height, color_type)
                                    },
                                    ImageFormat::PNG => {
                                        image::png::PNGEncoder::new(writer)
                                            .encode(texture.data.as_slice(), width, height, color_type)
                                    },
                                    ImageFormat::PPM => {
                                        image::ppm::PPMEncoder::new(&mut writer)
                                            .encode(texture.data.as_slice(), width, height, color_type)
                                    },
                                    _ => {
                                        throw!(AssetError::Unimplemented("Unsupported image format"));
                                    }
                                };

                                try_throw!(result);

                                return Ok(());
                            } else { throw!(AssetError::Unimplemented("3D texture exporting to non-Combustion image formats")); }
                        } else { throw!(AssetError::Unimplemented("Uneven or inapplicable bit depth image exporting to non-Combustion image formats")); }
                    } else { throw!(AssetError::Unimplemented("Saving compressed textures to non-Combustion image formats")); }
                } else { throw!(AssetError::Unimplemented("Saving multiple textures or cubemaps to non-Combustion image formats")); }
            }
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