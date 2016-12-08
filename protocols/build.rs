#[macro_use]
extern crate combustion_common as common;
extern crate capnpc;

use std::iter::FromIterator;
use std::io::prelude::*;
use std::env;
use std::fs::*;
use std::path::{Path, PathBuf};

/// Recursively visit directories
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) {
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() { visit_dirs(&path, cb); } else { cb(&entry); }
        }
    }
}

/// Visit directories, find .capnp files, compile them, then replace absolute module references with `super` in the output code.
fn compile_capnprotos(out_dir: String) {
    create_dir_all(out_dir.clone() + "/protocols").unwrap();

    visit_dirs(Path::new("src/protocols"), &|entry: &DirEntry| {
        if let Some(ext) = entry.path().as_path().extension() {
            if ext == "capnp" {
                info!("Attempting to compile: {:?} to {}", entry.path(), out_dir);

                if let Err(err) = capnpc::CompilerCommand::new().src_prefix("src").file(entry.path()).run() {
                    error!("Failed to compile protocol: {}", err);
                } else {
                    info!("Attempting to replace absolute module paths with `super`");

                    trace!("Generating out path");

                    let mut out_path = PathBuf::from_iter(entry.path().iter().skip(1));

                    let mut mod_name = out_path.file_name().unwrap().to_str().unwrap().to_string();

                    mod_name = mod_name.replace(".", "_");

                    trace!("Mod name: {}", mod_name);

                    out_path.set_file_name(mod_name.clone() + ".rs");

                    out_path = PathBuf::from(out_dir.clone()).join(out_path);

                    trace!("Opening output file {:?}...", out_path);

                    let mut out_file: File = File::open(out_path.clone()).unwrap();

                    let mut file_contents = String::new();

                    trace!("Reading output contents");
                    out_file.read_to_string(&mut file_contents).unwrap();

                    //Close file
                    trace!("Closing output file");
                    ::std::mem::drop(out_file);

                    trace!("Opening final output file {:?}...", out_path);
                    let mut out_file: File = File::create(out_path.clone()).unwrap();

                    trace!("Writing modified file contents...");
                    mod_name = format!("::{}", mod_name);

                    out_file.write_all(file_contents.replace(mod_name.as_str(), "super").as_bytes()).unwrap();

                    info!("Success!");
                }
            }
        }
    });

    info!("Cleaning up residual Cap'N Proto protocols directory");
    remove_dir_all("protocols").unwrap();
}

fn main() {
    common::log::init_global_logger("logs/build").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();

    info!("Compiling Cap'N Proto protocols");
    compile_capnprotos(out_dir.clone());
    info!("Finished all Cap'N Proto protocols");
}
