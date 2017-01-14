//! Event emitter implementation

use std::any::Any;

use fnv::FnvHashMap;

pub mod error;

#[cfg(feature = "parallel")]
pub mod parallel;

#[cfg(feature = "parallel")]
pub use self::parallel::*;

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
    /// Emit an event, invoking all the listeners for that event.
    fn emit<E: Into<String>>(&mut self, event: E) -> EventResult<usize>;
    /// Emit an event, invoking all the listeners for that event, and passing the given value to them.
    fn emit_value<T: Any, E: Into<String>>(&mut self, event: E, value: T) -> EventResult<usize>;
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
    pub fn add_listener_value<T: Any, E: Into<String>>(&mut self, event: E, mut cb: Box<FnMut(Option<&T>) -> EventResult<()>>) -> u64 {
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

    fn emit_value<T: Any, E: Into<String>>(&mut self, event: E, value: T) -> EventResult<usize> {
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

    fn emit_value<T: Any, E: Into<String>>(&mut self, event: E, value: T) -> EventResult<usize> {
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