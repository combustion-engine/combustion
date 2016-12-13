extern crate glfw;
extern crate nice_glfw;
extern crate image;
extern crate clap;
extern crate capnp;
extern crate capnpc;
extern crate glob;

#[macro_use]
extern crate combustion_common as common;
#[macro_use]
extern crate combustion_backend as backend;
extern crate combustion_protocols;

use combustion_protocols::protocols;

use common::error::*;
use common::utils;

use backend::gl::*;
use backend::gl::types::*;
use backend::gl::bindings as glb;

use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fs::File;

use clap::{App, Arg};
use glfw::WindowHint;
use image::{GenericImage, DynamicImage};

use protocols::texture;
use protocols::texture::protocol::{Kind};
use protocols::texture::protocol::texture as texture_protocol;
use protocols::texture::gl::*;

pub struct RawImage {
    pub dimensions: (u32, u32),
    pub data: Vec<u8>,
    pub channels: texture::Channels,
    pub out_path: PathBuf
}

pub fn read_texture<P: AsRef<Path> + Clone>(path: P) -> GLResult<RawImage> {
    let path = path.as_ref();

    if path.extension().unwrap() == texture::EXTENSION {
        //let mut out_path = path.to_path_buf();

        //TODO: Duplicate filenames need numbering, where the new number is ALWAYS greater than the previous one,
        //TODO: So start the number at the previous number if it exists
        /*
        let filename = path.file_stem().unwrap().to_string_lossy();
        let mut counter = 1;
        loop {
            out_path.set_file_name(format!("{}{}", filename, counter));
            if !out_path.exists() {break;}
            counter += 1;
        }*/

        let mut source = BufReader::new(File::open(path)?);

        let texture_message = capnp::serialize_packed::read_message(&mut source, capnp::message::ReaderOptions {
            traversal_limit_in_words: u64::max_value(), nesting_limit: 64
        }).expect_logged("Could not open Texture protocol");

        let texture = texture_message.get_root::<texture_protocol::Reader>()
                                         .expect_logged("No texture protocol root found");

        let width = texture.get_width();
        let height = texture.get_height();

        //TODO: Support more kinds
        //let depth = texture.get_depth();

        let kind = texture.get_kind()
                          .expect_logged("Couldn't find Kind value. This could be caused by using an older texture format.");

        //TODO: Support more kinds
        assert!(kind == Kind::Texture2D);

        let specific_format = texture::SpecificFormat::read_texture(&texture)
            .expect_logged("Error retrieving texture information");

        let data = texture.get_data()
                          .expect_logged("No texture data found");

        let generic_format = specific_format.to_generic();

        if specific_format.is_compressed() {
            info!("{:?} is compressed with {:?}", path, specific_format);
            info!("Decompressing...");

            unsafe {
                glb::CompressedTexImage2D(glb::TEXTURE_2D, 0, specific_format.specific(),
                                          width as GLsizei, height as GLsizei,
                                          0, data.len() as GLsizei, data.as_ptr() as *const _);
            }

            check_errors!();

            let len = width as usize * height as usize * generic_format.num_channels();

            let mut buffer: Vec<u8> = Vec::with_capacity(len);

            unsafe {
                buffer.set_len(len);

                glb::GetTexImage(glb::TEXTURE_2D, 0, generic_format.generic(), glb::UNSIGNED_BYTE, buffer.as_mut_ptr() as *mut _);
            }

            check_errors!();

            info!("Texture decompressed to {}", utils::human_readable::convert(len as f64));

            Ok(RawImage {
                dimensions: (width, height),
                data: buffer,
                channels: generic_format.channels,
                out_path: path.to_path_buf()
            })
        } else {
            Ok(RawImage {
                dimensions: (width, height),
                data: data.into(),
                channels: generic_format.channels,
                out_path: path.to_path_buf()
            })
        }
    } else {
        let image: DynamicImage = try!(image::open(path.clone()));

        let dimensions = image.dimensions();

        let (channels, data) = match image {
            DynamicImage::ImageLuma8(i) => (texture::Channels::R, i.into_raw()),
            DynamicImage::ImageLumaA8(i) => (texture::Channels::Rg, i.into_raw()),
            DynamicImage::ImageRgb8(i) => (texture::Channels::Rgb, i.into_raw()),
            DynamicImage::ImageRgba8(i) => (texture::Channels::Rgba, i.into_raw()),
        };

        Ok(RawImage {
            dimensions: dimensions,
            data: data,
            channels: channels,
            out_path: path.to_path_buf()
        })
    }
}

pub fn compress_texture<P: AsRef<Path> + Clone>(path: P, dir: &Path, matches: &clap::ArgMatches) -> GLResult<()> {
    info!("Loading {:?}", path.as_ref());

    let raw = try!(read_texture(path.clone()));

    let mut format = texture::GenericFormat {
        channels: raw.channels,
        srgb: matches.is_present("srgb"),
        float: matches.is_present("float"),
        signed: matches.is_present("signed"),
        blocksize: None,
        version: 0
    };

    let original_length = raw.data.len();

    let mut texture_message = capnp::message::Builder::new_default();

    {
        let mut texture_builder = texture_message.init_root::<texture_protocol::Builder>();

        texture_builder.set_width(raw.dimensions.0);
        texture_builder.set_height(raw.dimensions.1);

        //////////////////////

        let (compressed_data, specific_format) = if matches.is_present("none") {
            //Do absolutely nothing for none
            (raw.data, format.none())
        } else {
            format.signed = matches.is_present("signed");
            format.srgb = matches.is_present("srgb");
            format.float = matches.is_present("float");

            //////////////////////

            //Get the specific format we should use
            let specific_format = if matches.is_present("astc") {
                use texture::protocol::BlockSize;

                format.blocksize = Some(BlockSize::from_str(matches.value_of("blocksize").unwrap()));

                format.astc()
            } else if matches.is_present("s3tc") {
                format.version = matches.value_of("dxt_version").unwrap().parse().unwrap();

                format.s3tc()
            } else if matches.is_present("rgtc") {
                format.rgtc()
            } else if matches.is_present("bptc") {
                format.bptc()
            } else {
                format.none()
            };

            let mut internal_format = if matches.is_present("auto") {
                format.auto()
            } else {
                specific_format.specific()
            } as GLsizei;

            //////////////////////

            info!("Compressing {:?}...", path.as_ref());

            // Buffer the uncompressed texture to the GPU, letting OpenGL take care of the compression for us
            unsafe {
                glb::TexImage2D(glb::TEXTURE_2D, 0, internal_format,
                                raw.dimensions.0 as GLsizei, raw.dimensions.1 as GLsizei, 0,
                                format.generic(), glb::UNSIGNED_BYTE, raw.data.as_ptr() as *const _);
            }

            check_errors!();

            let mut compressed_size: GLsizei = 0;

            // Get the compressed size of the texture in bytes
            unsafe { glb::GetTexLevelParameteriv(glb::TEXTURE_2D, 0, glb::TEXTURE_COMPRESSED_IMAGE_SIZE, &mut compressed_size as *mut _); }
            check_errors!();

            // Get the real internal format that OpenGL decided to use
            unsafe { glb::GetTexLevelParameteriv(glb::TEXTURE_2D, 0, glb::TEXTURE_INTERNAL_FORMAT, &mut internal_format as *mut _); }
            check_errors!();

            // Preallocate buffer for compressed texture data
            let mut buffer: Vec<u8> = Vec::with_capacity(compressed_size as usize);

            unsafe {
                // Set length property before copying data into the buffer
                buffer.set_len(compressed_size as usize);

                // Copy compressed texture into buffer
                glb::GetCompressedTexImage(glb::TEXTURE_2D, 0, buffer.as_ptr() as *mut _);
            }
            check_errors!();

            // Done
            (buffer, texture::SpecificFormat::from_raw_gl(internal_format as GLenum))
        };

        info!("Texture successfully compressed using: {:?}", specific_format);

        let diff = original_length as f64 - compressed_data.len() as f64;

        info!("Original: {}, Compressed: {} - {:.2}% or {} difference",
              utils::human_readable::convert(original_length as f64),
              utils::human_readable::convert(compressed_data.len() as f64),
              diff / original_length as f64,
              utils::human_readable::convert(diff)
        );

        texture_builder.set_kind(Kind::Texture2D);

        specific_format.write_texture(&mut texture_builder);

        texture_builder.set_data(&compressed_data);
    }

    let path = raw.out_path;

    let stem = path.file_stem().unwrap();

    let mut out_path = PathBuf::from(dir).join(stem);

    out_path.set_extension(texture::EXTENSION);

    let mut out = try!(File::create(out_path.as_path()));

    try!(capnp::serialize_packed::write_message(&mut out, &texture_message));

    info!("Saved compressed texture to {:?}", out_path);

    Ok(())
}

fn main() {
    let possible_dxt_versions = ["1", "3", "5"];

    let possible_block_sizes = ["4x4", "5x4", "5x5", "6x5", "6x6", "8x5", "8x6", "10x5", "10x6", "8x8", "10x8", "10x10", "12x10", "12x12"];

    let not_none = ["auto", "rgtc", "bptc", "s3tc", "astc"];
    let not_auto = ["none", "rgtc", "bptc", "s3tc", "astc"];
    let not_rgtc = ["none", "auto", "bptc", "s3tc", "astc"];
    let not_bptc = ["none", "auto", "rgtc", "s3tc", "astc"];
    let not_s3tc = ["none", "auto", "rgtc", "bptc", "astc"];
    let not_astc = ["none", "auto", "rgtc", "bptc", "s3tc"];

    let app = App::new("texture_compressor")
        .version("0.1.0")
        .author("Aaron Trent <novacrazy@gmail.com>")
        .about("Converts image files into compressed textures")
        .arg(Arg::with_name("files").multiple(true).required(true).help("Images to compress").validator(|ref path| -> Result<(), String> {
            if Path::new(path).extension().is_some() { Ok(()) } else {
                Err("the images must have file extensions".to_string())
            }
        }))
        .arg(Arg::with_name("out_dir").short("o").help("Output directory").takes_value(true))
        .arg(Arg::with_name("none").long("none").display_order(0).conflicts_with_all(&not_none).required_unless_one(&not_none).help("Store uncompressed texture"))
        .arg(Arg::with_name("auto").long("auto").display_order(1).conflicts_with_all(&not_auto).required_unless_one(&not_auto).help("Determine best compression method automatically"))
        .arg(Arg::with_name("rgtc").long("rgtc").display_order(2).conflicts_with_all(&not_rgtc).required_unless_one(&not_rgtc).help("Use Red-Green compression algorithms"))
        .arg(Arg::with_name("bptc").long("bptc").display_order(3).conflicts_with_all(&not_bptc).required_unless_one(&not_bptc).help("Use BPTC compression algorithms"))
        .arg(Arg::with_name("s3tc").long("s3tc").display_order(4).conflicts_with_all(&not_s3tc).required_unless_one(&not_s3tc).requires("dxt_version").help("Use S3TC/DXT compression algorithms").alias("dxt"))
        .arg(Arg::with_name("astc").long("astc").display_order(5).conflicts_with_all(&not_astc).required_unless_one(&not_astc).requires("blocksize").help("Use ASTC block compression algorithm"))

        .arg(Arg::with_name("srgb").long("srgb").display_order(6).conflicts_with("none").help("Assume the source texture is in sRGB format"))

        .arg(Arg::with_name("blocksize").long("blocksize").display_order(7).takes_value(true).requires("astc").possible_values(&possible_block_sizes).help("Block size to use for the ASTC algorithm"))
        .arg(Arg::with_name("dxt_version").long("dxt").display_order(8).takes_value(true).requires("s3tc").possible_values(&possible_dxt_versions).help("DXT version to use with S3TC algorithm"))
        .arg(Arg::with_name("float").long("float").display_order(9).requires("bptc").help("Use floating point BPTC compression"))
        .arg(Arg::with_name("signed").long("signed").display_order(10).conflicts_with_all(&["auto", "none"]).help("Use signed formats when applicable"));

    let matches: clap::ArgMatches = app.get_matches();

    if matches.is_present("blocksize") && !matches.is_present("astc") {
        panic!("Blocksize must only be used with astc");
    }

    if matches.is_present("dxt") && !matches.is_present("s3tc") {
        panic!("dxt must only be used to s3tc");
    }

    if let Some(files) = matches.values_of("files") {
        common::log::init_global_logger("logs").expect("Could not initialize logging system!");

        let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect_logged("Could not initialize GLFW!");

        let (mut window, _) = nice_glfw::WindowBuilder::new(&mut glfw)
            .try_modern_context_hints()
            .size(1, 1)
            .common_hints(&[
                WindowHint::Visible(false),
                WindowHint::OpenGlDebugContext(true)
            ])
            .title("texture_compressor")
            .create()
            .expect_logged("Couldn't create window");

        info!("OpenGL context created");

        //Load up all the OpenGL functions from the process
        backend::gl::bindings::load_all_with(|symbol| window.get_proc_address(symbol) as *const _);

        //Enable debugging of OpenGL messages
        backend::gl::enable_debug(backend::gl::default_debug_callback, true).unwrap();

        //Debug message: (131202): Texture state performance warning: emulating compressed format not supported in hardware with decompressed images
        backend::gl::gl_debug::DEBUG_IGNORED.write().unwrap().push(131202);

        // Generate a new plain 2D texture to reuse for all compressions
        let _ = GLTexture::new(GLTextureKind::Texture2D).unwrap();

        let out_dir = matches.value_of("out_dir").map(|d| Path::new(d));

        let mut count = 0;

        for entry in files.flat_map(|entry| glob::glob(entry).unwrap()) {
            match entry {
                Ok(ref file) if file.exists() => {
                    let dir = if let Some(out_dir) = out_dir { out_dir } else {
                        file.parent().unwrap_or(Path::new("."))
                    };

                    compress_texture(file.clone(), dir, &matches).expect_logged("Could not process file");

                    count += 1;
                }
                Ok(ref file) => {
                    error!("Could not find file: {:?}", file);
                }
                Err(err) => {
                    error!("{}", err)
                }
            }
        }

        if count == 0 {
            error!("Could not find any input files matching the given paths or globs");
        }
    }
}
