#![feature(box_syntax)]

#[macro_use]
extern crate trace_error;

extern crate combustion_common as common;
extern crate combustion_protocols as protocols;
extern crate combustion_asset as asset;

use std::path::Path;
use std::sync::Arc;

use common::vfs;

use asset::asset::{Asset, AssetMedium};
use asset::assets::model;

fn main() {
    // Create the virtual filesystem handles
    let read_vfs = Arc::new(box vfs::mmap::MmapFS as vfs::BoxedVFS);
    let write_vfs = Arc::new(box vfs::default::DefaultFS as vfs::BoxedVFS);

    // Indicate an appropriate asset medium
    let load_medium = AssetMedium::File(Path::new("examples/sphere.dae"), read_vfs.clone());
    let save_medium = AssetMedium::File(Path::new("examples/sphere.bc"), write_vfs.clone());

    // Load the model asset
    let model = model::ModelAsset::load(load_medium, Default::default()).unwrap();

    // Display debug information for the model
    println!("{:?}", *model);

    model.save(save_medium, model::ModelAssetSaveArgs {
        storage_args: protocols::model::storage::ModelSaveArgs {
            mesh_args: protocols::mesh::storage::MeshSaveArgs {
                raw: true,
            }
        },
        pretty: true,
    }).unwrap();
}