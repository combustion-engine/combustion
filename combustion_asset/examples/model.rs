#![feature(box_syntax)]

#[macro_use]
extern crate trace_error;

extern crate combustion_protocols as protocols;
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
    let load_medium = AssetMedium::File(Path::new("examples/sphere.dae"), vfs.clone());
    let save_medium = AssetMedium::File(Path::new("examples/sphere.cmodel"), vfs.clone());

    // Load the model asset
    let model = model::ModelAsset::load(load_medium, Default::default()).unwrap();

    // Display debug information for the model
    println!("{:?}", *model);

    model.save(save_medium, model::ModelSaveArgs {
        storage_args: protocols::model::storage::ModelSaveArgs {
            mesh_args: protocols::mesh::storage::MeshSaveArgs {
                raw: true,
            }
        }
    }).unwrap();
}