extern crate combustion_protocols as protocols;
extern crate serde_yaml;

use std::fs::File;

use protocols::material::MaterialMap;
use protocols::material::sample::sample as sample_material;

#[test]
pub fn yaml_test() {
    use serde_yaml::to_string;

    println!("MaterialMap {}", to_string(&sample_material()).unwrap());
}

#[test]
pub fn parse_test() {
    use serde_yaml::from_reader;

    let src = File::open("tests/material.yaml").unwrap();

    let parsed: MaterialMap = from_reader(src).unwrap();

    println!("{:?}", parsed);
}