#![feature(box_syntax)]

#[macro_use]
extern crate trace_error;

extern crate combustion_asset as asset;

use asset::asset::{Asset, AssetMedium};
use asset::vfs;
use asset::assets::model;

use std::path::Path;
use std::sync::Arc;

fn main() {
    // Create the virtual filesystem handle
    let vfs = Arc::new(box vfs::default::DefaultFS as vfs::BoxedVFS);

    // Indicate an appropriate asset medium
    let medium = AssetMedium::File(Path::new("examples/sphere.dae"), vfs.clone());

    // Load the model asset
    let model = model::ModelAsset::load(medium, Default::default()).unwrap();

    // Display debug information for the model
    println!("{:?}", *model);
}