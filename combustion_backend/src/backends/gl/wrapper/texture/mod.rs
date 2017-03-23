use super::bindings::types::*;
use super::bindings::*;
use super::{GLObject, GLBindable};

use std::mem;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use image::{self, DynamicImage, GenericImage};

use super::error::*;
use super::shader::*;

pub mod dimensions;

pub use self::dimensions::{GLDimensions, GLOneDimension, GLTwoDimensions, GLThreeDimensions};

#[derive(Copy, Clone, Debug)]
pub enum GLTextureFilter {
    Linear,
    Nearest
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GLTextureKind {
    Texture1D = TEXTURE_1D,
    Texture2D = TEXTURE_2D,
    Texture3D = TEXTURE_3D,
    Rectangle = TEXTURE_RECTANGLE,
    BufferTexture = TEXTURE_BUFFER,
    Cubemap = TEXTURE_CUBE_MAP,
    Texture1DArray = TEXTURE_1D_ARRAY,
    Texture2DArray = TEXTURE_2D_ARRAY,
    CubemapArray = TEXTURE_CUBE_MAP_ARRAY,
    Texture2DMultisample = TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = TEXTURE_2D_MULTISAMPLE_ARRAY,
}

impl GLTextureKind {
    pub fn dimensions(&self) -> usize {
        match *self {
            GLTextureKind::Texture1D |
            GLTextureKind::BufferTexture |
            GLTextureKind::Texture1DArray => {
                1
            }
            GLTextureKind::Texture2D |
            GLTextureKind::Rectangle |
            GLTextureKind::Texture2DArray |
            GLTextureKind::Texture2DMultisample |
            GLTextureKind::Texture2DMultisampleArray => {
                2
            }
            GLTextureKind::Texture3D |
            GLTextureKind::Cubemap |
            GLTextureKind::CubemapArray => {
                3
            }
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum GLTextureWrap {
    ClampToEdge = CLAMP_TO_EDGE,
    ClampToBorder = CLAMP_TO_BORDER,
    MirroredRepeat = MIRRORED_REPEAT,
    Repeat = REPEAT,
    MirrorClampToEdge = MIRROR_CLAMP_TO_EDGE,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum GLCubemapFace {
    Right = TEXTURE_CUBE_MAP_POSITIVE_X,
    Left = TEXTURE_CUBE_MAP_NEGATIVE_X,
    Top = TEXTURE_CUBE_MAP_POSITIVE_Y,
    Bottom = TEXTURE_CUBE_MAP_NEGATIVE_Y,
    Back = TEXTURE_CUBE_MAP_POSITIVE_Z,
    Front = TEXTURE_CUBE_MAP_NEGATIVE_Z
}
/*
#[derive(Copy, Clone, Debug)]
pub struct GLCubemapPaths<P: AsRef<Path> + Copy> {
    pub right: P,
    pub left: P,
    pub top: P,
    pub bottom: P,
    pub back: P,
    pub front: P
}

impl<P: AsRef<Path> + Copy> GLCubemapPaths<P> {
    pub fn load_into(&self, texture: &mut GLTexture) -> GLResult<()> {
        try_rethrow!(texture.load_from_file(self.right, Some(GLCubemapFace::Right)));
        try_rethrow!(texture.load_from_file(self.left, Some(GLCubemapFace::Left)));
        try_rethrow!(texture.load_from_file(self.top, Some(GLCubemapFace::Top)));
        try_rethrow!(texture.load_from_file(self.bottom, Some(GLCubemapFace::Bottom)));
        try_rethrow!(texture.load_from_file(self.back, Some(GLCubemapFace::Back)));
        try_rethrow!(texture.load_from_file(self.front, Some(GLCubemapFace::Front)));

        Ok(())
    }
}*/

#[derive(Debug, Eq, PartialEq)]
pub struct GLBaseTexture {
    handle: GLuint,
    format: Option<GLenum>,
    internal_format: Option<GLenum>,
}

impl super::GLObject for GLBaseTexture {
    #[inline(always)]
    fn raw(&self) -> GLuint { self.handle }

    #[inline(always)]
    fn into_raw(mut self) -> GLuint {
        mem::replace(&mut self.handle, 0)
    }

    #[inline(always)]
    fn is_valid(&self) -> bool {
        TRUE == unsafe { IsTexture(self.handle) }
    }
}

impl GLBaseTexture {
    #[inline(always)]
    pub fn format(&self) -> Option<GLenum> { self.format }

    #[inline(always)]
    pub fn internal_format(&self) -> Option<GLenum> { self.internal_format }

    fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteTextures(1, &mut self.handle as *mut GLuint); }

            check_gl_errors!();
        }

        Ok(())
    }
}

impl Drop for GLBaseTexture {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLBaseTexture")
    }
}

macro_rules! declare_gl_texture {
    ($name:ident, $kind:ident, $dim:ident) => {
        pub struct $name(GLBaseTexture);

        impl $name {
            pub fn new() -> GLResult<$name> {
                let mut texture: GLuint = 0;

                unsafe { GenTextures(1, &mut texture as *mut _); }

                check_gl_errors!();

                unsafe { BindTexture(GLTextureKind::$kind as GLenum, texture); }

                check_gl_errors!();

                Ok($name(GLBaseTexture {
                    handle: texture,
                    format: None,
                    internal_format: None,
                }))
            }
        }

        impl Deref for $name {
            type Target = GLBaseTexture;

            #[inline(always)]
            fn deref(&self) -> &GLBaseTexture { &self.0 }
        }

        impl DerefMut for $name {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut GLBaseTexture { &mut self.0 }
        }

        impl $crate::backends::gl::traits::GLBindable for $name {
            fn bind(&self) -> GLResult<()> {
                try_rethrow!(self.check());

                unsafe { BindTexture(GLTextureKind::$kind as GLenum, self.raw()); }

                check_gl_errors!();

                Ok(())
            }
        }

        impl From<$name> for GLTexture {
            fn from(t: $name) -> GLTexture {
                GLTexture::$kind(t)
            }
        }

        impl GLGenericTexture for $name {}

        impl GLTextureVariant for $name {
            #[inline(always)]
            fn kind(&self) -> GLTextureKind {
                GLTextureKind::$kind
            }
        }

        impl GLDimensionalTexture<$dim> for $name {}
    }
}

pub enum GLTexture {
    Texture1D(GLTexture1D),
    Texture2D(GLTexture2D),
    Texture3D(GLTexture3D),
    Rectangle(GLRectangle),
    BufferTexture(GLBufferTexture),
    Cubemap(GLCubemap),
    Texture1DArray(GLTexture1DArray),
    Texture2DArray(GLTexture2DArray),
    CubemapArray(GLCubemapArray),
    Texture2DMultisample(GLTexture2DMultisample),
    Texture2DMultisampleArray(GLTexture2DMultisampleArray),
}

declare_gl_texture!(GLTexture1D,                 Texture1D,                 GLOneDimension);
declare_gl_texture!(GLTexture2D,                 Texture2D,                 GLTwoDimensions);
declare_gl_texture!(GLTexture3D,                 Texture3D,                 GLThreeDimensions);
declare_gl_texture!(GLRectangle,                 Rectangle,                 GLTwoDimensions);
declare_gl_texture!(GLBufferTexture,             BufferTexture,             GLOneDimension);
declare_gl_texture!(GLCubemap,                   Cubemap,                   GLThreeDimensions);
declare_gl_texture!(GLTexture1DArray,            Texture1DArray,            GLOneDimension);
declare_gl_texture!(GLTexture2DArray,            Texture2DArray,            GLTwoDimensions);
declare_gl_texture!(GLCubemapArray,              CubemapArray,              GLThreeDimensions);
declare_gl_texture!(GLTexture2DMultisample,      Texture2DMultisample,      GLTwoDimensions);
declare_gl_texture!(GLTexture2DMultisampleArray, Texture2DMultisampleArray, GLTwoDimensions);

impl Deref for GLTexture {
    type Target = GLBaseTexture;

    fn deref(&self) -> &GLBaseTexture {
        match *self {
            GLTexture::Texture1D(ref texture) => texture.deref(),
            GLTexture::Texture2D(ref texture) => texture.deref(),
            GLTexture::Texture3D(ref texture) => texture.deref(),
            GLTexture::Rectangle(ref texture) => texture.deref(),
            GLTexture::BufferTexture(ref texture) => texture.deref(),
            GLTexture::Cubemap(ref texture) => texture.deref(),
            GLTexture::Texture1DArray(ref texture) => texture.deref(),
            GLTexture::Texture2DArray(ref texture) => texture.deref(),
            GLTexture::CubemapArray(ref texture) => texture.deref(),
            GLTexture::Texture2DMultisample(ref texture) => texture.deref(),
            GLTexture::Texture2DMultisampleArray(ref texture) => texture.deref(),
        }
    }
}

impl DerefMut for GLTexture {
    fn deref_mut(&mut self) -> &mut GLBaseTexture {
        match *self {
            GLTexture::Texture1D(ref mut texture) => texture.deref_mut(),
            GLTexture::Texture2D(ref mut texture) => texture.deref_mut(),
            GLTexture::Texture3D(ref mut texture) => texture.deref_mut(),
            GLTexture::Rectangle(ref mut texture) => texture.deref_mut(),
            GLTexture::BufferTexture(ref mut texture) => texture.deref_mut(),
            GLTexture::Cubemap(ref mut texture) => texture.deref_mut(),
            GLTexture::Texture1DArray(ref mut texture) => texture.deref_mut(),
            GLTexture::Texture2DArray(ref mut texture) => texture.deref_mut(),
            GLTexture::CubemapArray(ref mut texture) => texture.deref_mut(),
            GLTexture::Texture2DMultisample(ref mut texture) => texture.deref_mut(),
            GLTexture::Texture2DMultisampleArray(ref mut texture) => texture.deref_mut(),
        }
    }
}

impl GLBindable for GLTexture {
    fn bind(&self) -> GLResult<()> {
        match *self {
            GLTexture::Texture1D(ref texture) => texture.bind(),
            GLTexture::Texture2D(ref texture) => texture.bind(),
            GLTexture::Texture3D(ref texture) => texture.bind(),
            GLTexture::Rectangle(ref texture) => texture.bind(),
            GLTexture::BufferTexture(ref texture) => texture.bind(),
            GLTexture::Cubemap(ref texture) => texture.bind(),
            GLTexture::Texture1DArray(ref texture) => texture.bind(),
            GLTexture::Texture2DArray(ref texture) => texture.bind(),
            GLTexture::CubemapArray(ref texture) => texture.bind(),
            GLTexture::Texture2DMultisample(ref texture) => texture.bind(),
            GLTexture::Texture2DMultisampleArray(ref texture) => texture.bind(),
        }
    }
}

impl GLTextureVariant for GLTexture {
    fn kind(&self) -> GLTextureKind {
        match *self {
            GLTexture::Texture1D(ref texture) => texture.kind(),
            GLTexture::Texture2D(ref texture) => texture.kind(),
            GLTexture::Texture3D(ref texture) => texture.kind(),
            GLTexture::Rectangle(ref texture) => texture.kind(),
            GLTexture::BufferTexture(ref texture) => texture.kind(),
            GLTexture::Cubemap(ref texture) => texture.kind(),
            GLTexture::Texture1DArray(ref texture) => texture.kind(),
            GLTexture::Texture2DArray(ref texture) => texture.kind(),
            GLTexture::CubemapArray(ref texture) => texture.kind(),
            GLTexture::Texture2DMultisample(ref texture) => texture.kind(),
            GLTexture::Texture2DMultisampleArray(ref texture) => texture.kind(),
        }
    }
}

impl GLGenericTexture for GLTexture {}

pub trait GLGenericTexture: Deref<Target=GLBaseTexture> + DerefMut + GLBindable + GLTextureVariant {
    fn generate_mipmap(&mut self) -> GLResult<()> {
        try_rethrow!(self.bind());

        unsafe { GenerateMipmap(self.kind() as GLenum); }

        check_gl_errors!();

        Ok(())
    }

    fn get_max_anisotropy(&mut self) -> GLResult<f32> {
        try_rethrow!(self.bind());

        let mut max_anisoptopy: GLfloat = 0.0;

        unsafe { GetFloatv(MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_anisoptopy as *mut _); }

        check_gl_errors!();

        Ok(max_anisoptopy)
    }

    fn set_anisotropy(&mut self, value: f32) -> GLResult<()> {
        try_rethrow!(self.bind());

        unsafe { TexParameterf(self.kind() as GLenum, TEXTURE_MAX_ANISOTROPY_EXT, value); }

        check_gl_errors!();

        Ok(())
    }

    fn set_filter(&mut self, filter: GLTextureFilter, mipmap: Option<GLTextureFilter>) -> GLResult<()> {
        try_rethrow!(self.bind());

        let min_filter;
        let mag_filter;

        match filter {
            GLTextureFilter::Linear => {
                min_filter = match mipmap {
                    None => LINEAR,
                    Some(GLTextureFilter::Nearest) => LINEAR_MIPMAP_NEAREST,
                    Some(GLTextureFilter::Linear) => LINEAR_MIPMAP_LINEAR
                } as GLint;

                mag_filter = LINEAR as GLint;
            }
            GLTextureFilter::Nearest => {
                min_filter = match mipmap {
                    None => NEAREST,
                    Some(GLTextureFilter::Nearest) => NEAREST_MIPMAP_NEAREST,
                    Some(GLTextureFilter::Linear) => NEAREST_MIPMAP_LINEAR
                } as GLint;

                mag_filter = NEAREST as GLint;
            }
        }

        unsafe {
            TexParameteri(self.kind() as GLenum, TEXTURE_MIN_FILTER, min_filter);
            TexParameteri(self.kind() as GLenum, TEXTURE_MAG_FILTER, mag_filter);
        }

        check_gl_errors!();

        Ok(())
    }
}

pub trait GLTextureVariant {
    fn kind(&self) -> GLTextureKind;
}

pub trait GLDimensionalTexture<D: GLDimensions>: Deref<Target=GLBaseTexture> + DerefMut + GLBindable + GLTextureVariant {
    fn set_wrap(&mut self, mode: GLTextureWrap) -> GLResult<()> {
        try_rethrow!(D::iterate(|dim| {
            self.set_wrap_dim(mode, dim)
        }));

        Ok(())
    }

    fn set_wrap_dim(&mut self, mode: GLTextureWrap, dim: D) -> GLResult<()> {
        try_rethrow!(self.bind());

        unsafe {
            TexParameteri(self.kind() as GLenum, dim.texture_wrap(), mode as GLint);
        }

        check_gl_errors!();

        Ok(())
    }
}

/*
impl GLTexture {

    pub fn set_active(&mut self, _index: u32) -> GLResult<()> {
        Ok(()) //TODO
    }

    pub fn load_empty(&mut self, width: usize, height: usize,
                      format: GLenum,
                      internal_format: GLenum) -> GLResult<()> {
        try_rethrow!(self.bind());

        let dims = self.kind.dimensions();

        if dims == 2 {
            unsafe {
                TexImage2D(self.kind as GLenum,
                           0,
                           internal_format as GLint,
                           width as GLsizei,
                           height as GLsizei,
                           0,
                           format,
                           FLOAT,
                           ptr::null());
            }
        }

        check_gl_errors!();

        self.format = Some(format);
        self.internal_format = Some(internal_format);

        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P, face: Option<GLCubemapFace>) -> GLResult<()> {
        try_rethrow!(self.check());

        let texture: DynamicImage = try_throw!(image::open(path));

        try_rethrow!(self.bind());

        let (width, height) = texture.dimensions();

        let (format, iformat, data) = match texture {
            DynamicImage::ImageLuma8(i) => (RED, R8, i.into_vec()),
            DynamicImage::ImageLumaA8(i) => (RG, RG8, i.into_vec()),
            DynamicImage::ImageRgb8(i) => (RGB, RGB8, i.into_vec()),
            DynamicImage::ImageRgba8(i) => (RGBA, RGBA8, i.into_vec())
        };

        let dims = self.kind.dimensions();

        //TODO: Support 1D textures
        if dims == 2 {
            unsafe {
                TexImage2D(self.kind as GLenum,
                           0,
                           iformat as GLint,
                           width as GLsizei,
                           height as GLsizei,
                           0,
                           format,
                           UNSIGNED_BYTE,
                           data.as_ptr() as *const _
                );
            }
        } else if dims == 3 {
            //TODO: Support more texture kinds
            match self.kind {
                GLTextureKind::Cubemap => {
                    let face = face.unwrap_or(GLCubemapFace::Right);

                    unsafe {
                        TexImage2D(face as GLenum,
                                   0,
                                   iformat as GLint,
                                   width as GLsizei,
                                   height as GLsizei,
                                   0,
                                   format,
                                   UNSIGNED_BYTE,
                                   data.as_ptr() as *const _
                        );
                    }
                }
                _ => {
                    unimplemented!();
                }
            }
        }

        self.format = Some(format);
        self.internal_format = Some(iformat);

        check_gl_errors!();

        Ok(())
    }
}*/