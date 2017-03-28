use std::fmt::Debug;

use ::error::BackendResult;

use ::resource::Resource;
use ::resource::texture::{TextureResource, TextureFilter};

use ::backends::gl::wrapper::texture;

impl From<TextureFilter> for texture::GLTextureFilter {
    fn from(filter: TextureFilter) -> texture::GLTextureFilter {
        match filter {
            TextureFilter::Linear => texture::GLTextureFilter::Linear,
            TextureFilter::Nearest => texture::GLTextureFilter::Nearest,
        }
    }
}

impl<T> Resource for T where T: texture::GLGenericTexture + Sized + Debug {}

impl<T> TextureResource for T where T: Resource + texture::GLGenericTexture {
    #[inline]
    fn generate_mipmaps(&mut self) -> BackendResult<()> {
        Ok(try_rethrow!(<Self as texture::GLGenericTexture>::generate_mipmaps(self)))
    }

    #[inline]
    fn get_max_anisotropy(&self) -> BackendResult<f32> {
        Ok(try_rethrow!(<Self as texture::GLGenericTexture>::get_max_anisotropy(self)))
    }

    #[inline]
    fn set_anisotropy(&mut self, value: f32) -> BackendResult<()> {
        Ok(try_rethrow!(<Self as texture::GLGenericTexture>::set_anisotropy(self, value)))
    }

    fn set_filtering(&mut self, filter: TextureFilter, mipmap: Option<TextureFilter>) -> BackendResult<()> {
        Ok(try_rethrow!(<Self as texture::GLGenericTexture>::set_filtering(self, filter.into(), mipmap.map(|filter| filter.into()))))
    }
}