//! Data structures for manipulating textures

use ::texture::protocol::TextureKind;
use ::blob::Blob;

use super::format::SpecificFormat;

/// Represents the variations of textures that can be used
#[derive(Clone, Serialize, Deserialize)]
pub enum RootTexture {
    /// Cubemap variant
    Cubemap(Cubemap),
    /// Single texture variant
    Single(Texture),
    /// Array of textures variant
    Array(Vec<Texture>),
}

/// Texture dimensions
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
    /// Constructor for `Dimensions` structure
    pub fn new(width: u32, height: u32, depth: u32) -> Dimensions {
        Dimensions { width: width, height: height, depth: depth }
    }
}

/// Represents a single texture
#[derive(Clone, Serialize, Deserialize)]
pub struct Texture {
    /// Binary texture data
    pub data: Blob,
    /// Texture dimensions
    pub dimensions: Dimensions,
    /// Texture kind
    pub kind: TextureKind,
    /// Storage format
    pub format: SpecificFormat,
}

/// Represents a cubemap made of six unique textures
#[derive(Clone, Serialize, Deserialize)]
pub struct Cubemap {
    /// Right texture
    pub right: Texture,
    /// Left texture
    pub left: Texture,
    /// Top texture
    pub top: Texture,
    /// Bottom texture
    pub bottom: Texture,
    /// Back texture
    pub back: Texture,
    /// Front texture
    pub front: Texture,
}