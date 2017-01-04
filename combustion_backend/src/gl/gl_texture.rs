use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;
use std::path::Path;

use image::{self, DynamicImage, GenericImage};

use super::gl_error::*;
use super::gl_shader::*;

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
    Buffer = TEXTURE_BUFFER,
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
            GLTextureKind::Buffer |
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
        try!(texture.load_from_file(self.right, Some(GLCubemapFace::Right)));
        try!(texture.load_from_file(self.left, Some(GLCubemapFace::Left)));
        try!(texture.load_from_file(self.top, Some(GLCubemapFace::Top)));
        try!(texture.load_from_file(self.bottom, Some(GLCubemapFace::Bottom)));
        try!(texture.load_from_file(self.back, Some(GLCubemapFace::Back)));
        try!(texture.load_from_file(self.front, Some(GLCubemapFace::Front)));

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GLTexture {
    handle: GLuint,
    pub kind: GLTextureKind,
    format: Option<GLenum>,
    internal_format: Option<GLenum>,
}

impl super::GLObject for GLTexture {
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

impl GLTexture {
    pub fn new(kind: GLTextureKind) -> GLResult<GLTexture> {
        let mut texture: GLuint = 0;

        unsafe { GenTextures(1, &mut texture as *mut _); }

        check_gl_errors!();

        unsafe { BindTexture(kind as GLenum, texture); }

        check_gl_errors!();

        Ok(GLTexture {
            handle: texture,
            kind: kind,
            format: None,
            internal_format: None,
        })
    }

    #[inline(always)]
    pub fn format(&self) -> Option<GLenum> { self.format }

    #[inline(always)]
    pub fn internal_format(&self) -> Option<GLenum> { self.internal_format }

    pub fn bind(&self) -> GLResult<()> {
        try!(self.check());

        unsafe { BindTexture(self.kind as GLenum, self.handle); }

        check_gl_errors!();

        Ok(())
    }

    pub fn set_active(&mut self, _index: u32) -> GLResult<()> {
        Ok(()) //TODO
    }

    pub fn load_empty(&mut self, width: usize, height: usize,
                      format: GLenum,
                      internal_format: GLenum) -> GLResult<()> {
        try!(self.bind());

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
        try!(self.check());

        let texture: DynamicImage = try!(image::open(path));

        try!(self.bind());

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

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteTextures(1, &mut self.handle as *mut GLuint); }

            check_gl_errors!();
        }

        Ok(())
    }

    pub fn generate_mipmap(&mut self) -> GLResult<()> {
        try!(self.bind());

        unsafe { GenerateMipmap(self.kind as GLenum); }

        check_gl_errors!();

        Ok(())
    }

    pub fn get_max_anisotropy(&mut self) -> GLResult<f32> {
        try!(self.bind());

        let mut max_anisoptopy: GLfloat = 0.0;

        unsafe { GetFloatv(MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_anisoptopy as *mut _); }

        check_gl_errors!();

        Ok(max_anisoptopy)
    }

    pub fn set_anisotropy(&mut self, value: f32) -> GLResult<()> {
        try!(self.bind());

        unsafe { TexParameterf(self.kind as GLenum, TEXTURE_MAX_ANISOTROPY_EXT, value); }

        check_gl_errors!();

        Ok(())
    }

    pub fn set_filter(&mut self, filter: GLTextureFilter, mipmap: Option<GLTextureFilter>) -> GLResult<()> {
        try!(self.bind());

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
            TexParameteri(self.kind as GLenum, TEXTURE_MIN_FILTER, min_filter);
            TexParameteri(self.kind as GLenum, TEXTURE_MAG_FILTER, mag_filter);
        }

        check_gl_errors!();

        Ok(())
    }

    pub fn set_wrap(&mut self, mode: GLTextureWrap) -> GLResult<()> {
        for dim in 0..self.kind.dimensions() {
            try!(self.set_wrap_dim(mode, dim));
        }

        Ok(())
    }

    pub fn set_wrap_dim(&mut self, mode: GLTextureWrap, dim: usize) -> GLResult<()> {
        try!(self.bind());

        if dim < self.kind.dimensions() {
            unsafe {
                TexParameteri(self.kind as GLenum, match dim {
                    0 => TEXTURE_WRAP_S,
                    1 => TEXTURE_WRAP_T,
                    2 => TEXTURE_WRAP_R,
                    _ => unreachable!()
                }, mode as GLint);
            }

            check_gl_errors!();

            Ok(())
        } else {
            Err(GLError::InvalidValue)
        }
    }
}

impl Drop for GLTexture {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLTexture")
    }
}