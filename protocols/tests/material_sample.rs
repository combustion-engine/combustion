extern crate combustion_protocols as protocols;
extern crate serde_hjson;

use protocols::material::sample::sample as sample_material;

#[test]
pub fn json_test() {
    use serde_hjson::to_string;

    println!("MaterialMap {}", to_string(&sample_material()).unwrap());
}