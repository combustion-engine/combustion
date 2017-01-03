#[macro_use]
extern crate combustion_common as common;

#[inline(never)]
fn test2() {
    println!("{}", backtrace![]);
}

#[inline(never)]
fn test() {
    test2();
}

#[inline(never)]
fn main() {
    test();
}