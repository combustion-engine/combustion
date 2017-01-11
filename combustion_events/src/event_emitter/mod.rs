// Use std::any for type erasure
use std::any::Any;
// Use a fast hashing algorithm for the event lookup table
use fnv::FnvHashMap;

pub mod error;

pub use self::error::*;

/// Define the internal callback type, which is a boxed closure that accepts optional arguments
type Callback = Box<FnMut(Option<&Box<Any>>) -> EventResult<()>>;

/// Stores the listener callback and its ID value
struct EventListener {
    id: u64,
    cb: Callback,
}

impl EventListener {
    /// Create a new `EventListener` from an id and callback
    #[inline(always)]
    fn new(id: u64, cb: Callback) -> EventListener {
        EventListener { id: id, cb: cb }
    }
}

/// Defines methods for emitting events
pub trait AbstractEventEmitter {
    /// Emit an event, invoked all the listeners for that event.
    fn emit<E: Into<String>>(&mut self, event: E) -> EventResult<usize>;
    /// Emit an event, invoked all the listeners for that event, and passing the given value to them.
    fn emit_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, value: T) -> EventResult<usize>;
}

/// Standard Event Emitter
///
/// Holds type-erased callbacks that are associated with an event `String`.
/// When an event is emitted, callbacks associated with it are invoked.
pub struct EventEmitter {
    events: FnvHashMap<String, Vec<EventListener>>,
    counter: u64,
}

impl EventEmitter {
    /// Create a new empty `EventEmitter`
    pub fn new() -> EventEmitter {
        EventEmitter { events: FnvHashMap::default(), counter: 0 }
    }

    // The "real" implementation of add_listener that generates the IDs and inserts it into the table
    fn add_listener_impl(&mut self, event: String, cb: Callback) -> u64 {
        let mut listeners = self.events.entry(event).or_insert_with(|| Vec::new());

        let id = self.counter;

        self.counter += 1;

        listeners.push(EventListener::new(id, cb));

        id
    }

    /// Add a simple listener callback that does not accept any arguments
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    #[inline]
    pub fn add_listener<E: Into<String>>(&mut self, event: E, mut cb: Box<FnMut() -> EventResult<()>>) -> u64 {
        self.add_listener_impl(event.into(), Box::new(move |_| -> EventResult<()> { cb() }))
    }

    /// Add a listener that can accept a reference to a value passed via `emit`
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    pub fn add_listener_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, mut cb: Box<FnMut(Option<&T>) -> EventResult<()>>) -> u64 {
        self.add_listener_impl(event.into(), Box::new(move |arg| -> EventResult<()> {
            if let Some(arg) = arg.as_ref() { cb(arg.downcast_ref::<T>()) } else { cb(None) }
        }))
    }

    /// Removes a listener with the given ID and associated with the given event.
    ///
    /// If the listener was not found (either doesn't exist or the wrong event given) `false` is returned.
    ///
    /// If the listener was removed, `true` is returned.
    pub fn remove_listener<E: Into<String>>(&mut self, event: E, id: u64) -> bool {
        if let Some(mut listeners) = self.events.get_mut(&event.into()) {
            let index = listeners.binary_search_by_key(&id, |listener| listener.id);

            if let Ok(index) = index {
                listeners.remove(index);

                return true;
            }
        }

        false
    }

    /// Exhaustively searches through ALL events for a listener with the given ID.
    ///
    /// `false` is returned if it was not found.
    pub fn remove_any_listener(&mut self, id: u64) -> bool {
        for (_, mut listeners) in self.events.iter_mut() {
            let index = listeners.binary_search_by_key(&id, |listener| listener.id);

            if let Ok(index) = index {
                listeners.remove(index);

                return true;
            }
        }

        false
    }

    /// Create a short-lived proxy object for a single event.
    ///
    /// This avoids hash table lookups for every call to `emit`
    pub fn proxy<'a, E: Into<String>>(&'a mut self, event: E) -> EventEmitterProxy<'a> {
        let event = event.into();

        let mut listeners = self.events.entry(event.clone()).or_insert_with(|| Vec::new());

        EventEmitterProxy {
            event: event,
            listeners: listeners
        }
    }
}

/// A short-lived proxy object for a single event.
///
/// This avoids hash table lookups for every call to `emit`
pub struct EventEmitterProxy<'a> {
    event: String,
    listeners: &'a mut Vec<EventListener>,
}

impl<'a> EventEmitterProxy<'a> {
    /// Get the associated event for this proxy
    #[inline]
    pub fn event(&self) -> &String {
        &self.event
    }
}

impl AbstractEventEmitter for EventEmitter {
    fn emit<E: Into<String>>(&mut self, event: E) -> EventResult<usize> {
        if let Some(mut listeners) = self.events.get_mut(&event.into()) {
            let mut count = 0;

            for listener in listeners.iter_mut() {
                try_rethrow!((listener.cb)(None));

                count += 1;
            }

            Ok(count)
        } else {
            Ok(0)
        }
    }

    fn emit_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, value: T) -> EventResult<usize> {
        if let Some(mut listeners) = self.events.get_mut(&event.into()) {
            // Box `T` and cast `T` to `Any` here
            let boxed: Option<Box<Any>> = Some(Box::new(value));
            // Cast `Option<Box<Any>>` to `Option<&Box<Any>>`
            let boxed_ref = boxed.as_ref();

            let mut count = 0;

            for listener in listeners.iter_mut() {
                // Only use a reference to the boxed value in the loop for performance
                try_rethrow!((listener.cb)(boxed_ref));

                count += 1;
            }

            Ok(count)
        } else {
            Ok(0)
        }
    }
}

impl<'a> AbstractEventEmitter for EventEmitterProxy<'a> {
    fn emit<E: Into<String>>(&mut self, event: E) -> EventResult<usize> {
        let event = event.into();

        if event == self.event {
            let mut count = 0;

            for listener in self.listeners.iter_mut() {
                try_rethrow!((listener.cb)(None));

                count += 1;
            }

            Ok(count)
        } else {
            Ok(0)
        }
    }

    fn emit_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, value: T) -> EventResult<usize> {
        let event = event.into();

        if event == self.event {
            // Box `T` and cast `T` to `Any` here
            let boxed: Option<Box<Any>> = Some(Box::new(value));
            // Cast `Option<Box<Any>>` to `Option<&Box<Any>>`
            let boxed_ref = boxed.as_ref();

            let mut count = 0;

            for listener in self.listeners.iter_mut() {
                // Only use a reference to the boxed value in the loop for performance
                try_rethrow!((listener.cb)(boxed_ref));

                count += 1;
            }

            Ok(count)
        } else {
            Ok(0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test::Bencher;

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
    fn backtrace() {
        // This should print out something like:
        //
        // Unspecified error
        // Stack backtrace for task "<event_emitter::test::backtrace>" at line 350 of "src\event_emitter/mod.rs":
        //    0:     0x7ff6f2f9a930 - combustion_events::event_emitter::test::backtrace::{{closure}}
        //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:350
        //    1:     0x7ff6f2f95820 - combustion_events::event_emitter::{{impl}}::add_listener::{{closure}}<&str>
        //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:68
        //    2:     0x7ff6f2f960b0 - combustion_events::event_emitter::{{impl}}::emit<&str>
        //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:153
        //    3:     0x7ff6f2f9a620 - combustion_events::event_emitter::test::backtrace
        //                          at E:\code\projects\Combustion\combustion_events\src\event_emitter\mod.rs:353
        //    4:     0x7ff6f2fac0b0 - test::{{impl}}::call_box<(),closure>
        //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libtest\lib.rs:140
        //    5:     0x7ff6f30071b0 - panic_unwind::__rust_maybe_catch_panic
        //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libpanic_unwind\lib.rs:98
        //    6:     0x7ff6f2f9dfe0 - std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()>
        //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libstd\panicking.rs:456
        //    7:     0x7ff6f30071b0 - panic_unwind::__rust_maybe_catch_panic
        //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libpanic_unwind\lib.rs:98
        //    8:     0x7ff6f2fa6890 - alloc::boxed::{{impl}}::call_box<(),closure>
        //                          at C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\liballoc\boxed.rs:605
        //    9:     0x7ff6f3001670 - std::sys::imp::thread::{{impl}}::new::thread_start
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
}