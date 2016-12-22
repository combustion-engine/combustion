extern crate combustion_protocols as protocols;
extern crate serde_json;

use protocols::scene::sample::sample as sample_scene;

#[test]
pub fn json_test() {
    use serde_json::to_string_pretty;

    println!("Scene {}", to_string_pretty(&sample_scene()).unwrap());
}