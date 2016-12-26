use std::any::Any;
use fnv::FnvHashMap;
use smallvec::SmallVec;

type SmallVec16<T> = SmallVec<[T; 16]>;

type Callback = Box<FnMut(Option<&Box<Any>>)>;

struct EventListener {
    id: u64,
    cb: Callback,
}

impl EventListener {
    #[inline(always)]
    fn new(id: u64, cb: Callback) -> EventListener {
        EventListener { id: id, cb: cb }
    }
}

/// Defines methods for emitting events
pub trait AbstractEventEmitter {
    /// Emit an event, invoked all the listeners for that event.
    fn emit<E: Into<String>>(&mut self, event: E) -> usize;
    /// Emit an event, invoked all the listeners for that event, and passing the given value to them.
    fn emit_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, value: T) -> usize;
}

/// Standard Event Emitter
///
/// Holds type-erased callbacks that are associated with an event `String`.
/// When an event is emitted, callbacks associated with it are invoked.
pub struct EventEmitter {
    events: FnvHashMap<String, SmallVec16<EventListener>>,
    counter: u64,
}

impl EventEmitter {
    /// Create a new empty `EventEmitter`
    pub fn new() -> EventEmitter {
        EventEmitter { events: FnvHashMap::default(), counter: 0 }
    }

    fn add_listener_impl(&mut self, event: String, cb: Callback) -> u64 {
        let mut listeners = self.events.entry(event).or_insert_with(|| SmallVec16::new());

        let id = self.counter;

        self.counter += 1;

        listeners.push(EventListener::new(id, cb));

        id
    }

    /// Add a simple listener callback that does not accept any arguments
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    #[inline]
    pub fn add_listener<E: Into<String>>(&mut self, event: E, mut cb: Box<FnMut()>) -> u64 {
        self.add_listener_impl(event.into(), Box::new(move |_| { cb() }))
    }

    /// Add a listener that can accept a reference to a value passed via `emit`
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    pub fn add_listener_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, mut cb: Box<FnMut(Option<&T>)>) -> u64 {
        self.add_listener_impl(event.into(), Box::new(move |arg| {
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
            let index = listeners.iter().position(|listener| listener.id == id);

            if let Some(index) = index {
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
            let index = listeners.iter().position(|listener| listener.id == id);

            if let Some(index) = index {
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

        let mut listeners = self.events.entry(event.clone()).or_insert_with(|| SmallVec16::new());

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
    listeners: &'a mut SmallVec16<EventListener>,
}

impl<'a> EventEmitterProxy<'a> {
    /// Get the associated event for this proxy
    #[inline]
    pub fn event(&self) -> &String { &self.event }
}

impl AbstractEventEmitter for EventEmitter {
    fn emit<E: Into<String>>(&mut self, event: E) -> usize {
        if let Some(mut listeners) = self.events.get_mut(&event.into()) {
            listeners.iter_mut().map(|listener| (listener.cb)(None)).count()
        } else { 0 }
    }

    fn emit_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, value: T) -> usize {
        if let Some(mut listeners) = self.events.get_mut(&event.into()) {
            // Box `T` and cast `T` to `Any` here
            let boxed: Option<Box<Any>> = Some(Box::new(value));
            // Cast `Option<Box<Any>>` to `Option<&Box<Any>>`
            let boxed_ref = boxed.as_ref();
            // Only use a reference to the boxed value in the loop for performance
            listeners.iter_mut().map(|listener| (listener.cb)(boxed_ref)).count()
        } else { 0 }
    }
}

impl<'a> AbstractEventEmitter for EventEmitterProxy<'a> {
    fn emit<E: Into<String>>(&mut self, event: E) -> usize {
        let event = event.into();

        if event == self.event {
            self.listeners.iter_mut().map(|listener| (listener.cb)(None)).count()
        } else { 0 }
    }

    fn emit_value<T: 'static + Copy, E: Into<String>>(&mut self, event: E, value: T) -> usize {
        let event = event.into();

        if event == self.event {
            // Box `T` and cast `T` to `Any` here
            let boxed: Option<Box<Any>> = Some(Box::new(value));
            // Cast `Option<Box<Any>>` to `Option<&Box<Any>>`
            let boxed_ref = boxed.as_ref();
            // Only use a reference to the boxed value in the loop for performance
            self.listeners.iter_mut().map(|listener| (listener.cb)(boxed_ref)).count()
        } else { 0 }
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
        });

        emitter.add_listener_value::<i32, _>("Test", box |arg| {
            println!("{:?}", arg);
        });
    }

    #[test]
    fn test_emit() {
        let mut emitter = EventEmitter::new();

        emitter.add_listener("test", box || {});

        emitter.add_listener_value::<i32, _>("test", box |arg| {
            println!("{:?}", arg);
        });

        emitter.add_listener_value::<&str, _>("test", box |arg| {
            println!("{:?}", arg);
        });

        assert_eq!(emitter.emit("test"), 3);
        assert_eq!(emitter.emit_value("test", 10), 3);
        assert_eq!(emitter.emit_value("test", "test"), 3);
    }

    #[test]
    fn test_proxy() {
        let mut emitter = EventEmitter::new();

        emitter.add_listener("test", box || {});

        emitter.add_listener_value::<i32, _>("test", box |arg| {
            println!("{:?}", arg);
        });

        emitter.add_listener_value::<&str, _>("test", box |arg| {
            println!("{:?}", arg);
        });

        {
            let mut proxy = emitter.proxy("test");

            assert_eq!(proxy.emit("test"), 3);
            assert_eq!(proxy.emit_value("test", 10), 3);
            assert_eq!(proxy.emit_value("test", "test"), 3);
        }

        // Compile time test to see if the proxy lifetime ended
        emitter.add_listener("test", box || {});
    }

    #[bench]
    fn bench_emit(b: &mut Bencher) {
        let mut emitter = EventEmitter::new();


        for i in 0..1000 {
            emitter.add_listener(format!("test{}", i), box || {});
        }

        b.iter(|| emitter.emit("test"))
    }

    #[bench]
    fn bench_proxy(b: &mut Bencher) {
        let mut emitter = EventEmitter::new();

        for i in 0..1000 {
            emitter.add_listener(format!("test{}", i), box || {});
        }

        let mut proxy = emitter.proxy("test");

        b.iter(|| proxy.emit("test"))
    }
}