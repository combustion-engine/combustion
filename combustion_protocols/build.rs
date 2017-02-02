#[macro_use]
extern crate combustion_common as common;
extern crate capnpc;
extern crate cmake;
extern crate gcc;
extern crate walkdir;

#[cfg(feature = "cpp")]
use std::process::Command;

use walkdir::*;

use common::ext::*;

/// Visit directories, find .capnp files, compile them, then replace absolute module references with `super` in the output code.
fn compile_capnprotos() {
    info!("Compiling Cap'N Proto protocols");

    let walker = walkdir::WalkDir::new("protocols").into_iter().filter_entry(|entry| {
        // Skip files without the "capnp" extension, but allow other items through
        match entry.path().extension() {
            Some(ext) if ext != "capnp" => false,
            _ => true
        }
    });

    for entry in walker {
        let entry = entry.unwrap();

        // Skip dirs and symlinks
        if !entry.file_type().is_file() {
            continue;
        }

        info!("Attempting to generate: {:?} as Rust", entry.path());

        capnpc::CompilerCommand::new().file(entry.path()).import_path("protocols").run().expect_logged("Failed to compile protocol");

        #[cfg(feature = "cpp")]
        {
            info!("Attempting to generate: {:?} as C++", entry.path());
            let output = Command::new("capnp.exe").arg("compile").arg("-oc++").arg("-Iprotocols").arg(entry.path()).output()
                                                  .expect_logged("Failed to compile protocol");

            if !output.status.success() {
                error!("Output: {:?}", output);
                panic!("Output: {:?}", output);
            }
        }

        info!("Success!");
    }

    #[cfg(feature = "cpp")]
    {
        info!("Moving generated C++ code to cpp directory");

        Command::new("mv")
            .arg("protocols/*.capnp.h")
            .arg("protocols/*.capnp.c++")
            .arg("protocols/*.capnp.cpp")
            .arg("cpp")
            .output()
            .expect_logged("Could not move files");

        info!("Success!");
    }

    info!("Finished all Cap'N Proto protocols");
}

#[cfg(feature = "cpp")]
fn build_cpp() {
    use std::env;

    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    info!("Compiling Cap'N Proto C++ static library");

    let mut capnp_config = cmake::Config::new("cpp/capnproto/c++");

    capnp_config.define("CAPNP_LITE", "1");
    capnp_config.define("EXTERNAL_CAPNP", "1");
    capnp_config.define("BUILD_TOOLS", "OFF");
    capnp_config.define("BUILD_TESTING", "OFF");

    capnp_config.define("CAPNP_EXECUTABLE", format!("{}/capnp.exe", src_dir));
    capnp_config.define("CAPNP_CXX_EXECUTABLE", format!("{}/capnpc-c++.exe", src_dir));
    capnp_config.define("CAPNP_INCLUDE_DIRECTORY", format!("{}/cpp/capnproto/c++/src/capnp/", src_dir));

    let capnp_dst = capnp_config.build();

    println!("cargo:rustc-link-search=native={}", capnp_dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=capnp");
    println!("cargo:rustc-link-lib=static=kj");

    Command::new("cp").arg("-R")
                      .arg(format!("{}/include/*", out_dir))
                      .arg("cpp/include/")
                      .output()
                      .expect_logged("Failed to copy include files");

    info!("Success!");
}

fn main() {
    common::log::init_global_logger("logs/build").unwrap();

    compile_capnprotos();

    #[cfg(feature = "cpp")]
    build_cpp();
}
