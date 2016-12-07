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
    create_dir_all(out_dir + "/protocols").unwrap();

    visit_dirs(Path::new("src"), &|entry: &DirEntry| {
        if let Some(ext) = entry.path().as_path().extension() {
            if ext == "capnp" {
                capnpc::CompilerCommand::new()
                    .file(entry.path())
                    .src_prefix("src")
                    .run()
                    .expect("compiling schema");
            }
        }
    });
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    compile_capnprotos(out_dir.clone());
}