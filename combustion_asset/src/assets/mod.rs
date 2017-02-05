//! All assets which can be loaded and saved

pub mod texture;
pub mod model;

/// TODO
pub enum GenericAsset {
    /// Texture asset
    Texture(texture::TextureAsset),
    /// Model asset
    Model(model::ModelAsset),
}