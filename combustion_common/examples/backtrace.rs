#[macro_use]
extern crate combustion_common as common;

use common::error::*;

use std::io::{self, ErrorKind};

fn from_error() -> Result<(), io::Error> {
    Err(io::ErrorKind::AddrInUse.into())
}

fn rethrow2() -> TraceResult<(), io::Error> {
    try_throw!(from_error());

    Ok(())
}

fn errorable() -> TraceResult<i32, io::Error> {
    throw!(ErrorKind::AlreadyExists);

    Ok(43)
}

fn rethrow() -> TraceResult<i32, io::Error> {
    let res = try_rethrow!(errorable());

    println!("{}", res);

    Ok(res)
}

#[inline(never)]
fn test2() {
    if let Err(err) = rethrow() {
        println!("{:?}", err);
    }

    if let Err(err) = rethrow2() {
        println!("{:?}", err);
    }
}

#[inline(never)]
fn test() {
    test2();

    println!("{}", backtrace!());
}

#[inline(never)]
fn main() {
    test();
}