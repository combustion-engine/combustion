extern crate assimp;
extern crate clap;
extern crate capnp;
extern crate capnpc;
extern crate glob;

#[macro_use]
extern crate combustion_common as common;
extern crate combustion_protocols;

use combustion_protocols::protocols;

use common::error::*;
use common::utils;

use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fs::File;

use clap::{App, Arg};

fn main() {
    let app = App::new("model_converter")
        .version("0.1.0")
        .author("Aaron Trent <novacrazy@gmail.com>")
        .about("Converts various model formats into compressed Combustion models")
        .arg(Arg::with_name("files").multiple(true).required(true).help("Models to convert").validator(|ref path| -> Result<(), String> {
            if Path::new(path).extension().is_some() { Ok(()) } else {
                Err("the files must have file extensions".to_string())
            }
        }))
        .arg(Arg::with_name("scene").long("scene").short("s").help("Store any scene information in an accompanying JSON file"))
    ;

    let matches = app.get_matches();
}
