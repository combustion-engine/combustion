//! All assets which can be loaded and saved

pub mod texture;

/// TODO
pub enum GenericAsset {
    /// Texture asset
    Texture(texture::TextureAsset)
}