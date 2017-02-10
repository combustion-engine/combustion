//! Data structures for manipulating textures

use ::texture::protocol::TextureKind;
use ::blob::Blob;

use super::format::SpecificFormat;

/// Represents the variations of textures that can be used
#[derive(Clone, Serialize, Deserialize)]
pub enum RootTexture {
    /// Cubemap variant
    Cubemap(Box<Cubemap>),
    /// Single texture variant
    Single(Box<Texture>),
    /// Array of textures variant
    Array(Vec<Texture>),
}

impl RootTexture {
    /// Checks if any textures in `RootTexture` are compressed
    pub fn has_compressed(&self) -> bool {
        match *self {
            RootTexture::Single(ref texture) => texture.is_compressed(),
            RootTexture::Cubemap(ref cubemap) => cubemap.any_compressed(),
            RootTexture::Array(ref array) => array.iter().any(|texture| texture.is_compressed())
        }
    }
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

    /// Returns `(width, height, depth)` tuple
    pub fn to_tuple(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.depth)
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

impl Texture {
    /// Checks if the texture is in a compressed format
    pub fn is_compressed(&self) -> bool {
        self.format.is_compressed()
    }
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

impl Cubemap {
    /// Checks if any texture in the cubemap is compressed
    pub fn any_compressed(&self) -> bool {
        self.right.is_compressed()
            || self.left.is_compressed()
            || self.top.is_compressed()
            || self.bottom.is_compressed()
            || self.back.is_compressed()
            || self.front.is_compressed()
    }

    /// Checks if all textures in the cubemap are compressed
    pub fn all_compressed(&self) -> bool {
        self.right.is_compressed()
            && self.left.is_compressed()
            && self.top.is_compressed()
            && self.bottom.is_compressed()
            && self.back.is_compressed()
            && self.front.is_compressed()
    }
}