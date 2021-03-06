#![feature(box_syntax)]

#[macro_use]
extern crate trace_error;

extern crate combustion_common as common;
extern crate combustion_asset as asset;

use common::vfs;

use asset::asset::{Asset, AssetMedium};
use asset::assets::texture;

use std::path::Path;
use std::sync::Arc;

fn main() {
    // Create the virtual filesystem handle
    let vfs = Arc::new(box vfs::default::DefaultFS as vfs::BoxedVFS);

    let load_medium = AssetMedium::File(Path::new("examples/uv_test_512.png"), vfs.clone());
    let save_medium = AssetMedium::File(Path::new("examples/uv_test_512_2.png"), vfs.clone());
    let save_medium2 = AssetMedium::File(Path::new("examples/uv_test_512.json"), vfs.clone());

    let texture = texture::TextureAsset::load(load_medium, Default::default()).unwrap();

    texture.save(save_medium, Default::default()).unwrap();
    texture.save(save_medium2, texture::TextureAssetSaveArgs {
        pretty: true,
        ..Default::default()
    }).unwrap();
}