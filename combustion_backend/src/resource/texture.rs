use ::error::BackendResult;

use super::Resource;

pub enum TextureFilter {
    Linear,
    Nearest
}

pub trait TextureResource: Resource {
    fn generate_mipmaps(&mut self) -> BackendResult<()>;
    fn get_max_anisotropy(&self) -> BackendResult<f32>;
    fn set_anisotropy(&mut self, value: f32) -> BackendResult<()>;
    fn set_filtering(&mut self, filter: TextureFilter, mipmap: Option<TextureFilter>) -> BackendResult<()>;
}