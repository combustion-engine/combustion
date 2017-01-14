#![cfg(feature = "bugs")]
#![feature(test, box_syntax, conservative_impl_trait)]

extern crate test;
extern crate thread_id;

#[macro_use]
extern crate combustion_common as common;
extern crate combustion_events as events;

extern crate futures;

use std::thread;

use futures::*;

fn test() -> impl Future<Item = u32, Error = ()> {
    futures::finished(0)
}

fn test2() -> Result<impl Future<Item = u32, Error = ()>, ()> {
    test()
}

#[test]
fn bugs() {
    let a = box test();
    let b = box futures::finished(()).and_then(test2).flatten();

    let c = a.join(b);

    println!("{:?}", c.wait().unwrap());
}