extern crate combustion_game as game;

use game::core;

use core::common::utils::*;
use core::common::utils::human_readable::*;

fn main() {
    println!("{}", humanize_si(1032.5));
}