#![feature(test, box_syntax)]

extern crate test;

#[macro_use]
extern crate combustion_common as common;
extern crate combustion_events as events;

use test::Bencher;

use events::event_emitter::*;

#[test]
fn test_add_listener() {
    let mut emitter = EventEmitter::new();

    let test = 10;

    emitter.add_listener("Test", box move || {
        println!("Test: {}", test);
        Ok(())
    });

    emitter.add_listener_value::<i32, _>("Test", box |arg| {
        println!("{:?}", arg);
        Ok(())
    });
}

#[test]
fn test_emit() {
    let mut emitter = EventEmitter::new();

    emitter.add_listener("test", box || {
        Ok(())
    });

    emitter.add_listener_value::<i32, _>("test", box |arg| {
        println!("{:?}", arg);
        Ok(())
    });

    emitter.add_listener_value::<&str, _>("test", box |arg| {
        println!("{:?}", arg);
        Ok(())
    });

    assert_eq!(emitter.emit("test").unwrap(), 3);
    assert_eq!(emitter.emit_value("test", 10).unwrap(), 3);
    assert_eq!(emitter.emit_value("test", "test").unwrap(), 3);
}

#[test]
fn test_remove_listener() {
    let mut emitter = EventEmitter::new();

    let a = emitter.add_listener("test", box || {
        println!("A");
        Ok(())
    });

    let b = emitter.add_listener_value::<i32, _>("test", box |arg| {
        println!("B {:?}", arg);
        Ok(())
    });

    emitter.add_listener_value::<&str, _>("test", box |arg| {
        println!("C {:?}", arg);
        Ok(())
    });

    assert_eq!(emitter.emit("test").unwrap(), 3);
    assert_eq!(emitter.emit_value("test", 10).unwrap(), 3);
    assert_eq!(emitter.emit_value("test", "test").unwrap(), 3);

    emitter.remove_listener("test", b);

    assert_eq!(emitter.emit("test").unwrap(), 2);
    assert_eq!(emitter.emit_value("test", 10).unwrap(), 2);
    assert_eq!(emitter.emit_value("test", "test").unwrap(), 2);

    emitter.remove_listener("test", a);

    assert_eq!(emitter.emit("test").unwrap(), 1);
    assert_eq!(emitter.emit_value("test", 10).unwrap(), 1);
    assert_eq!(emitter.emit_value("test", "test").unwrap(), 1);
}

#[test]
fn test_proxy() {
    let mut emitter = EventEmitter::new();

    emitter.add_listener("test", box || {
        Ok(())
    });

    emitter.add_listener_value::<i32, _>("test", box |arg| {
        println!("{:?}", arg);
        Ok(())
    });

    emitter.add_listener_value::<&str, _>("test", box |arg| {
        println!("{:?}", arg);
        Ok(())
    });

    {
        let mut proxy = emitter.proxy("test");

        assert_eq!(proxy.emit("test").unwrap(), 3);
        assert_eq!(proxy.emit_value("test", 10).unwrap(), 3);
        assert_eq!(proxy.emit_value("test", "test").unwrap(), 3);
    }

    // Compile time test to see if the proxy lifetime ended
    emitter.add_listener("test", box || {
        Ok(())
    });
}

#[test]
fn event_backtrace() {
    // This should print out something like:
    //
    // Unspecified error
    // Stack backtrace for task "<event_emitter::test::event_backtrace>" at line 377 of "src\event_emitter/mod.rs":
    //    0:     0x7ff6d728a930 - combustion_events::event_emitter::test::event_backtrace::{{closure}}
    //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:377
    //    1:     0x7ff6d7285820 - combustion_events::event_emitter::{{impl}}::add_listener::{{closure}}<&str>
    //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:68
    //    2:     0x7ff6d72860b0 - combustion_events::event_emitter::{{impl}}::emit<&str>
    //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:153
    //    3:     0x7ff6d728a620 - combustion_events::event_emitter::test::event_backtrace
    //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:380
    //    4:     0x7ff6d729c0b0 - test::{{impl}}::call_box<(),closure>
    //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libtest\lib.rs:140
    //    5:     0x7ff6d72f71b0 - panic_unwind::__rust_maybe_catch_panic
    //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libpanic_unwind\lib.rs:98
    //    6:     0x7ff6d728dfe0 - std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()>
    //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:456
    //    7:     0x7ff6d72f71b0 - panic_unwind::__rust_maybe_catch_panic
    //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libpanic_unwind\lib.rs:98
    //    8:     0x7ff6d7296890 - alloc::boxed::{{impl}}::call_box<(),closure>
    //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\liballoc\boxed.rs:605
    //    9:     0x7ff6d72f1670 - std::sys::imp::thread::{{impl}}::new::thread_start
    //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\sys\windows\thread.rs:50
    //    10:     0x7ffead558350 - BaseThreadInitThunk
    //                          at <anonymous>


    let mut emitter = EventEmitter::new();

    emitter.add_listener("test", box || {
        throw!(EventError::Unspecified);
    });

    match emitter.emit("test") {
        Err(trace) => {
            println!("{}", trace);
        },
        Ok(_) => panic!("Expected an error")
    }
}

#[bench]
fn bench_emit(b: &mut Bencher) {
    let mut emitter = EventEmitter::new();

    for i in 0..1000 {
        emitter.add_listener(format!("test{}", i), box || {
            Ok(())
        });
    }

    b.iter(|| emitter.emit("test").unwrap())
}

#[bench]
fn bench_proxy(b: &mut Bencher) {
    let mut emitter = EventEmitter::new();

    for i in 0..1000 {
        emitter.add_listener(format!("test{}", i), box || {
            Ok(())
        });
    }

    let mut proxy = emitter.proxy("test");

    b.iter(|| proxy.emit("test").unwrap())
}