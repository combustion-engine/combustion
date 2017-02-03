use ::texture::protocol::TextureKind;

use super::format::SpecificFormat;

#[derive(Clone, Serialize, Deserialize)]
pub enum RootTexture {
    Cubemap(Cubemap),
    Single(Texture),
    Array(Vec<Texture>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Texture depth
    pub depth: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32, depth: u32) -> Dimensions {
        Dimensions { width: width, height: height, depth: depth }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Texture {
    /// Binary texture data
    pub data: Vec<u8>,
    /// Texture dimensions
    pub dimensions: Dimensions,
    /// Texture kind
    pub kind: TextureKind,
    /// Storage format
    pub format: SpecificFormat,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cubemap {
    pub right: Texture,
    pub left: Texture,
    pub top: Texture,
    pub bottom: Texture,
    pub back: Texture,
    pub front: Texture,
}