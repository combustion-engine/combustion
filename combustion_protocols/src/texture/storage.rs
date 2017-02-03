use ::error::ProtocolResult;
use ::traits::{Storage, StorageQuery};

use super::data::{format, texture};
use super::data::texture::{Texture, RootTexture};
use super::protocol;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RootTextureQuery {
    Single,
    Cubemap,
    Array,
}

impl StorageQuery for RootTextureQuery {
    type Arguments = ();
    type Result = RootTextureQuery;
}

impl<'a> Storage<'a> for Texture {
    type Builder = protocol::texture::Builder<'a>;
    type Reader = protocol::texture::Reader<'a>;

    type LoadArgs = ();
    type SaveArgs = ();
    type Query = ();

    fn load_from_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<Texture> {
        let format = {
            let compression_reader = reader.get_compression();

            let which = match try_throw!(compression_reader.which()) {
                protocol::texture::compression::None(uncompressed_reader) => {
                    let uncompressed_reader = try_throw!(uncompressed_reader);

                    format::Which::None(format::Uncompressed {
                        channels: try_throw!(uncompressed_reader.get_format()),
                        data_type: try_throw!(uncompressed_reader.get_type()),
                    })
                },
                protocol::texture::compression::Rgtc(rgtc) => format::Which::Rgtc(try_throw!(rgtc)),
                protocol::texture::compression::Bptc(bptc) => format::Which::Bptc(try_throw!(bptc)),
                protocol::texture::compression::S3tc(s3tc) => format::Which::S3tc(try_throw!(s3tc)),
                protocol::texture::compression::Astc(blocksize) => format::Which::Astc(try_throw!(blocksize)),
            };

            let srgb = reader.get_srgb();

            format::SpecificFormat {
                which: which,
                srgb: srgb,
            }
        };

        let dimensions = {
            let dimensions_reader = reader.get_dimensions();

            texture::Dimensions {
                width: dimensions_reader.get_width(),
                height: dimensions_reader.get_height(),
                depth: dimensions_reader.get_depth(),
            }
        };

        Ok(Texture {
            data: try_throw!(reader.get_data()).into(),
            dimensions: dimensions,
            kind: try_throw!(reader.get_kind()),
            format: format,
        })
    }

    fn save_to_builder_args(&self, mut builder: Self::Builder, _: ()) -> ProtocolResult<()> {
        builder.set_kind(self.kind);

        {
            let mut dimensions_builder = builder.borrow().init_dimensions();

            dimensions_builder.set_width(self.dimensions.width);
            dimensions_builder.set_height(self.dimensions.height);
            dimensions_builder.set_depth(self.dimensions.depth);
        }

        builder.set_srgb(self.format.srgb);

        {
            let mut compression_builder = builder.borrow().init_compression();

            match self.format.which {
                format::Which::None(ref uncompressed) => {
                    let mut uncompressed_builder = compression_builder.init_none();

                    uncompressed_builder.set_format(uncompressed.channels);
                    uncompressed_builder.set_type(uncompressed.data_type);
                },
                format::Which::Rgtc(ref rgtc) => compression_builder.set_rgtc(*rgtc),
                format::Which::Bptc(ref bptc) => compression_builder.set_bptc(*bptc),
                format::Which::S3tc(ref s3tc) => compression_builder.set_s3tc(*s3tc),
                format::Which::Astc(ref blocksize) => compression_builder.set_astc(*blocksize),
            }
        }

        builder.set_data(self.data.as_slice());

        Ok(())
    }

    fn query_reader_args(_: Self::Reader, _: ()) -> ProtocolResult<()> {
        unimplemented!()
    }
}

impl<'a> Storage<'a> for RootTexture {
    type Builder = protocol::root_texture::Builder<'a>;
    type Reader = protocol::root_texture::Reader<'a>;

    type LoadArgs = ();
    type SaveArgs = ();
    type Query = RootTextureQuery;

    fn load_from_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<RootTexture> {
        let which_texture_reader = reader.get_texture();

        match try_throw!(which_texture_reader.which()) {
            protocol::root_texture::texture::Single(texture_reader) => {
                let texture_reader = try_throw!(texture_reader);

                let texture = Texture::load_from_reader(texture_reader)?;

                Ok(RootTexture::Single(texture))
            },
            protocol::root_texture::texture::Cubemap(cubemap_reader) => {
                let cubemap_reader = try_throw!(cubemap_reader);

                let right_reader = try_throw!(cubemap_reader.get_right());
                let left_reader = try_throw!(cubemap_reader.get_left());
                let top_reader = try_throw!(cubemap_reader.get_top());
                let bottom_reader = try_throw!(cubemap_reader.get_bottom());
                let back_reader = try_throw!(cubemap_reader.get_back());
                let front_reader = try_throw!(cubemap_reader.get_front());

                Ok(RootTexture::Cubemap(texture::Cubemap {
                    right: Texture::load_from_reader(right_reader)?,
                    left: Texture::load_from_reader(left_reader)?,
                    top: Texture::load_from_reader(top_reader)?,
                    bottom: Texture::load_from_reader(bottom_reader)?,
                    back: Texture::load_from_reader(back_reader)?,
                    front: Texture::load_from_reader(front_reader)?,
                }))
            },
            protocol::root_texture::texture::Array(array_reader) => {
                let array_reader = try_throw!(array_reader);

                let mut textures = Vec::with_capacity(array_reader.len() as usize);

                for texture_reader in array_reader.iter() {
                    textures.push(try_rethrow!(Texture::load_from_reader(texture_reader)));
                }

                Ok(RootTexture::Array(textures))
            }
        }
    }

    fn save_to_builder_args(&self, builder: Self::Builder, _: ()) -> ProtocolResult<()> {
        let texture_union_builder = builder.init_texture();

        match *self {
            RootTexture::Single(ref texture) => {
                texture.save_to_builder(texture_union_builder.init_single())
            },
            RootTexture::Cubemap(ref cubemap) => {
                let mut cubemap_builder = texture_union_builder.init_cubemap();

                try_rethrow!(cubemap.right.save_to_builder(cubemap_builder.borrow().init_right()));
                try_rethrow!(cubemap.left.save_to_builder(cubemap_builder.borrow().init_left()));
                try_rethrow!(cubemap.top.save_to_builder(cubemap_builder.borrow().init_top()));
                try_rethrow!(cubemap.bottom.save_to_builder(cubemap_builder.borrow().init_bottom()));
                try_rethrow!(cubemap.back.save_to_builder(cubemap_builder.borrow().init_back()));
                try_rethrow!(cubemap.front.save_to_builder(cubemap_builder.borrow().init_front()));

                Ok(())
            },
            RootTexture::Array(ref array) => {
                let mut array_builder = texture_union_builder.init_array(array.len() as u32);

                for (i, texture) in array.iter().enumerate() {
                    let texture_builder = array_builder.borrow().get(i as u32);

                    try_rethrow!(texture.save_to_builder(texture_builder));
                }

                Ok(())
            }
        }
    }

    fn query_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<RootTextureQuery> {
        let which_texture_reader = reader.get_texture();

        match try_throw!(which_texture_reader.which()) {
            protocol::root_texture::texture::Single(_) => {
                Ok(RootTextureQuery::Single)
            },
            protocol::root_texture::texture::Cubemap(_) => {
               Ok(RootTextureQuery::Cubemap)
            },
            protocol::root_texture::texture::Array(_) => {
                Ok(RootTextureQuery::Array)
            }
        }
    }
}