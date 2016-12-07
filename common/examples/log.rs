#[macro_use]
extern crate combustion_common as common;

use common::log::init_global_logger;

fn main() {
    init_global_logger("logs").unwrap();

    info!("Testing")
}