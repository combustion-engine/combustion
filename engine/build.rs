#[macro_use]
extern crate combustion_common as common;
extern crate capnpc;

use std::env;
use std::fs::*;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) {
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() { visit_dirs(&path, cb); } else { cb(&entry); }
        }
    }
}

fn compile_capnprotos(out_dir: String) {
    create_dir_all(out_dir.clone() + "/protocols").unwrap();

    visit_dirs(Path::new("src/protocols"), &|entry: &DirEntry| {
        if let Some(ext) = entry.path().as_path().extension() {
            if ext == "capnp" {
                info!("Attempting to compile: {:?} to {}", entry.path(), out_dir);

                if let Err(err) = capnpc::CompilerCommand::new().src_prefix("src").file(entry.path()).run() {
                    error!("Failed to compile protocol: {}", err);
                } else {
                    info!("Success!");
                }
            }
        }
    });
}

fn main() {
    common::log::init_global_logger("logs/build").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();

    info!("Compiling Cap'N Proto protocols");
    compile_capnprotos(out_dir.clone());
    info!("Finished all Cap'N Proto protocols");
}


