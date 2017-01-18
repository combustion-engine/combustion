#![cfg(feature = "parallel")]
#![feature(test, box_syntax)]

extern crate test;
extern crate thread_id;

#[macro_use]
extern crate combustion_common as common;
extern crate combustion_events as events;

extern crate futures;

use std::thread;

use futures::*;

use test::Bencher;

use events::event_emitter::parallel::*;

#[test]
fn test_pooled_emit() {
    let mut emitter = ParallelEventEmitter::new();

    emitter.add_listener("test", box || {
        thread::sleep_ms(1000);
        println!("Thread: {}", thread_id::get());
        Ok(())
    }).unwrap();

    emitter.add_listener_value::<i32, _>("test", box |arg| {
        thread::sleep_ms(1000);
        println!("Thread: {}, {:?}", thread_id::get(), arg);
        Ok(())
    }).unwrap();

    emitter.add_listener_value::<&str, _>("test", box |arg| {
        thread::sleep_ms(1000);
        println!("Thread: {}, {:?}", thread_id::get(), arg);
        Ok(())
    }).unwrap();

    let a = emitter.emit("test").boxed();
    let b = emitter.emit_value("test", 10).boxed();
    let c = emitter.emit_value("test", "test").boxed();

    let res = future::join_all(vec![a, b, c].into_iter()).wait().unwrap();

    assert_eq!(res, vec![3, 3, 3]);
}
