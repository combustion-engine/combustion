use ::error::{ProtocolResult, ProtocolError};

use super::data::{format, texture};
use super::protocol;

pub fn load_texture_from_reader(texture_reader: protocol::texture::Reader) -> ProtocolResult<texture::Texture> {
    let format = {
        let compression_reader = texture_reader.get_compression();

        let which = match try_throw!(compression_reader.which()) {
            protocol::texture::compression::None(uncompressed_reader) => {
                let uncompressed_reader = try_throw!(uncompressed_reader);

                format::Which::None(format::Uncompressed {
                    format: try_throw!(uncompressed_reader.get_format()),
                    data_type: try_throw!(uncompressed_reader.get_type()),
                })
            },
            protocol::texture::compression::Rgtc(rgtc) => format::Which::Rgtc(try_throw!(rgtc)),
            protocol::texture::compression::Bptc(bptc) => format::Which::Bptc(try_throw!(bptc)),
            protocol::texture::compression::S3tc(s3tc) => format::Which::S3tc(try_throw!(s3tc)),
            protocol::texture::compression::Astc(blocksize) => format::Which::Astc(try_throw!(blocksize)),
        };

        let srgb = texture_reader.get_srgb();

        format::SpecificFormat {
            which: which,
            srgb: srgb,
        }
    };

    let dimensions = {
        let dimensions_reader = texture_reader.get_dimensions();

        texture::Dimensions {
            width: dimensions_reader.get_width(),
            height: dimensions_reader.get_height(),
            depth: dimensions_reader.get_depth(),
        }
    };

    Ok(texture::Texture {
        data: try_throw!(texture_reader.get_data()).into(),
        dimensions: dimensions,
        kind: try_throw!(texture_reader.get_kind()),
        format: format,
    })
}

pub fn load_root_texture_from_reader(root_texture_reader: protocol::root_texture::Reader) -> ProtocolResult<texture::RootTexture> {
    let which_texture_reader = root_texture_reader.get_texture();

    match try_throw!(which_texture_reader.which()) {
        protocol::root_texture::texture::Single(texture_reader) => {
            let texture_reader = try_throw!(texture_reader);

            let texture = try_rethrow!(load_texture_from_reader(texture_reader));

            Ok(texture::RootTexture::Single(texture))
        },
        protocol::root_texture::texture::Cubemap(cubemap_reader) => {
            let cubemap_reader = try_throw!(cubemap_reader);

            let right_reader = try_throw!(cubemap_reader.get_right());
            let left_reader = try_throw!(cubemap_reader.get_left());
            let top_reader = try_throw!(cubemap_reader.get_top());
            let bottom_reader = try_throw!(cubemap_reader.get_bottom());
            let back_reader = try_throw!(cubemap_reader.get_back());
            let front_reader = try_throw!(cubemap_reader.get_front());

            let right = try_rethrow!(load_texture_from_reader(right_reader));
            let left = try_rethrow!(load_texture_from_reader(left_reader));
            let top = try_rethrow!(load_texture_from_reader(top_reader));
            let bottom = try_rethrow!(load_texture_from_reader(bottom_reader));
            let back = try_rethrow!(load_texture_from_reader(back_reader));
            let front = try_rethrow!(load_texture_from_reader(front_reader));

            Ok(texture::RootTexture::Cubemap(texture::Cubemap {
                right: right,
                left: left,
                top: top,
                bottom: bottom,
                back: back,
                front: front,
            }))
        },
    }
}

pub fn save_texture_to_builder(mut texture_builder: protocol::texture::Builder, texture: &texture::Texture) -> ProtocolResult<()> {
    texture_builder.set_kind(texture.kind);

    {
        let mut dimensions_builder = texture_builder.borrow().init_dimensions();

        dimensions_builder.set_width(texture.dimensions.width);
        dimensions_builder.set_height(texture.dimensions.height);
        dimensions_builder.set_depth(texture.dimensions.depth);
    }

    texture_builder.set_srgb(texture.format.srgb);

    {
        let mut compression_builder = texture_builder.borrow().init_compression();

        match texture.format.which {
            format::Which::None(ref uncompressed) => {
                let mut uncompressed_builder = compression_builder.init_none();

                uncompressed_builder.set_format(uncompressed.format);
                uncompressed_builder.set_type(uncompressed.data_type);
            },
            format::Which::Rgtc(ref rgtc) => compression_builder.set_rgtc(*rgtc),
            format::Which::Bptc(ref bptc) => compression_builder.set_bptc(*bptc),
            format::Which::S3tc(ref s3tc) => compression_builder.set_s3tc(*s3tc),
            format::Which::Astc(ref blocksize) => compression_builder.set_astc(*blocksize),
        }
    }

    texture_builder.set_data(texture.data.as_slice());

    Ok(())
}

pub fn save_root_texture_to_builder(root_texture_builder: protocol::root_texture::Builder, root_texture: &texture::RootTexture) -> ProtocolResult<()> {
    let texture_union_builder = root_texture_builder.init_texture();

    match *root_texture {
        texture::RootTexture::Single(ref texture) => {
            save_texture_to_builder(texture_union_builder.init_single(), texture)
        },
        texture::RootTexture::Cubemap(ref cubemap) => {
            let mut cubemap_builder = texture_union_builder.init_cubemap();

            try_rethrow!(save_texture_to_builder(cubemap_builder.borrow().init_right(), &cubemap.right));
            try_rethrow!(save_texture_to_builder(cubemap_builder.borrow().init_left(), &cubemap.left));
            try_rethrow!(save_texture_to_builder(cubemap_builder.borrow().init_top(), &cubemap.top));
            try_rethrow!(save_texture_to_builder(cubemap_builder.borrow().init_bottom(), &cubemap.bottom));
            try_rethrow!(save_texture_to_builder(cubemap_builder.borrow().init_back(), &cubemap.back));
            try_rethrow!(save_texture_to_builder(cubemap_builder.borrow().init_front(), &cubemap.front));

            Ok(())
        },
    }
}