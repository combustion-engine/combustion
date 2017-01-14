//! Parallel `PooledEventEmitter`
//!
//! The `PooledEventEmitter` object allows for event emitter listeners
//! to be invoked in a thread pool concurrently.
//!
//! Which is nice.

use std::any::Any;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use fnv::FnvHashMap;
use std::collections::hash_map::Entry;

use futures;
use futures::*;
use futures_cpupool::*;

use common::error::Trace;

use super::*;

/// Stores the listener callback and its ID value
struct SyncEventListener {
    id: u64,
    cb: RwLock<Callback>,
}

unsafe impl Send for SyncEventListener {}

unsafe impl Sync for SyncEventListener {}

impl SyncEventListener {
    /// Create a new `SyncEventListener` from an id and callback
    #[inline(always)]
    fn new(id: u64, cb: Callback) -> Arc<SyncEventListener> {
        Arc::new(SyncEventListener { id: id, cb: RwLock::new(cb) })
    }
}

type SyncEventListenerLock = Arc<SyncEventListener>;

type SyncListenersLock = Arc<RwLock<Vec<SyncEventListenerLock>>>;

/// Pooled Event Emitter
///
/// It behaves essentially like the standard `EventEmitter`,
/// but listeners are invoked in parallel in a thread pool.
pub struct PooledEventEmitter {
    inner: Arc<Inner>,
}

struct Inner {
    events: RwLock<FnvHashMap<String, SyncListenersLock>>,
    counter: AtomicUsize,
    pool: CpuPool,
}

unsafe impl Send for Inner {}

unsafe impl Sync for Inner {}

impl PooledEventEmitter {
    /// Creates a new `PooledEventEmitter` with the default `CpuPool`
    pub fn new() -> PooledEventEmitter {
        PooledEventEmitter::with_pool(CpuPool::new_num_cpus())
    }

    /// Creates a new `PooledEventEmitter` with an already existing `CpuPool` instance.
    ///
    /// This allows for custom thread preferences and lifecycle hooks.
    pub fn with_pool(pool: CpuPool) -> PooledEventEmitter {
        PooledEventEmitter {
            inner: Arc::new(Inner {
                events: RwLock::new(FnvHashMap::default()),
                counter: AtomicUsize::new(0),
                pool: pool,
            })
        }
    }

    fn add_listener_impl(&mut self, event: String, cb: Callback) -> EventResult<u64> {
        match try_throw!(self.inner.events.write()).entry(event) {
            Entry::Occupied(listeners_lock) => {
                let mut listeners = try_throw!(listeners_lock.get().write());

                let id = self.inner.counter.fetch_add(1, Ordering::Relaxed) as u64;

                listeners.push(SyncEventListener::new(id, cb));

                Ok(id)
            },
            Entry::Vacant(vacant) => {
                let mut listeners = Vec::with_capacity(1);

                let id = self.inner.counter.fetch_add(1, Ordering::Relaxed) as u64;

                listeners.push(SyncEventListener::new(id, cb));

                vacant.insert(Arc::new(RwLock::new(listeners)));

                Ok(id)
            }
        }
    }

    /// Add a simple listener callback that does not accept any arguments
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    #[inline]
    pub fn add_listener<E: Into<String>>(&mut self, event: E, cb: Box<Fn() -> EventResult<()>>) -> EventResult<u64> {
        self.add_listener_impl(event.into(), Box::new(move |_| -> EventResult<()> { cb() }))
    }

    /// Add a listener that can accept a reference to a value passed via `emit`
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    pub fn add_listener_value<T: Any + Clone, E: Into<String>>(&mut self, event: E, cb: Box<Fn(Option<&T>) -> EventResult<()>>) -> EventResult<u64> where T: Send {
        self.add_listener_impl(event.into(), Box::new(move |arg| -> EventResult<()> {
            if let Some(arg) = arg.as_ref() { cb(arg.downcast_ref::<T>()) } else { cb(None) }
        }))
    }

    /// Variation of `add_listener_value` that accepts `Sync` types, where intermediate copies on `emit` are unnecessary.
    ///
    /// There is nothing statically forcing the use of this instead of `add_listener_value`,
    /// but it is here just in case your type `T` is `Sync` but might not implement `Clone`
    ///
    /// The return value of this is a unique ID for that listener, which can later be used to remove it if desired.
    pub fn add_listener_sync<T: Any, E: Into<String>>(&mut self, event: E, cb: Box<Fn(Option<&T>) -> EventResult<()>>) -> EventResult<u64> where T: Send + Sync {
        self.add_listener_impl(event.into(), Box::new(move |arg| -> EventResult<()> {
            if let Some(arg) = arg.as_ref() { cb(arg.downcast_ref::<T>()) } else { cb(None) }
        }))
    }

    /// Removes a listener with the given ID and associated with the given event.
    ///
    /// If the listener was not found (either doesn't exist or the wrong event given) `Ok(false)` is returned.
    ///
    /// If the listener was removed, `Ok(true)` is returned.
    pub fn remove_listener<E: Into<String>>(&mut self, event: E, id: u64) -> EventResult<bool> {
        if let Some(listeners_lock) = try_throw!(self.inner.events.read()).get(&event.into()) {
            let mut listeners = try_throw!(listeners_lock.write());

            let index = listeners.binary_search_by_key(&id, |listener| listener.id);

            if let Ok(index) = index {
                listeners.remove(index);

                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Exhaustively searches through ALL events for a listener with the given ID.
    ///
    /// `Ok(false)` is returned if it was not found.
    pub fn remove_any_listener(&mut self, id: u64) -> EventResult<bool> {
        for (_, listeners_lock) in try_throw!(self.inner.events.read()).iter() {
            let mut listeners = try_throw!(listeners_lock.write());

            let index = listeners.binary_search_by_key(&id, |listener| listener.id);

            if let Ok(index) = index {
                listeners.remove(index);

                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Emit an event, invoking all the listeners for that event in the thread pool concurrently.
    ///
    /// The `Future` returned by `emit` resolves to the number of listeners invoked,
    /// and any errors should be forward up.
    pub fn emit<E: Into<String>>(&mut self, event: E) -> impl Future<Item = usize, Error = Trace<EventError>> {
        let event = event.into();
        let inner = self.inner.clone();

        self.inner.pool.spawn_fn(move || {
            if let Some(listeners_lock) = try_throw!(inner.events.read()).get(&event) {
                let listeners = try_throw!(listeners_lock.read());

                // Don't bother if there aren't any listeners to invoke anyway
                if listeners.len() > 0 {
                    let mut listener_futures = Vec::with_capacity(listeners.len());

                    for listener_lock in listeners.iter() {
                        // Clone a local copy of the listener_lock that can be sent to the spawn
                        let listener_lock = listener_lock.clone();

                        let listener_future = inner.pool.spawn_fn(move || -> EventResult<()> {
                            let mut cb_guard = try_throw!(listener_lock.cb.write());

                            // Force a mutable reference to the callback
                            try_rethrow!((&mut *cb_guard)(None));

                            Ok(())
                        });

                        listener_futures.push(listener_future);
                    }

                    // Join them all together into a single future and map the length of the results to the final future
                    return Ok(future::join_all(listener_futures)
                        .map(|executed: Vec<()>| executed.len()).boxed());
                }
            }

            Ok(futures::finished(0).boxed())
        }).flatten()
    }

    /// Emit an event, invoking all the listeners for that event in the thread pool concurrently.
    ///
    /// A copy of the value will be passed (by reference copy) to every listener.
    ///
    /// The `Future` returned by `emit` resolves to the number of listeners invoked,
    /// and any errors should be forward up.
    pub fn emit_value<T: Any + Clone, E: Into<String>>(&mut self, event: E, value: T) -> impl Future<Item = usize, Error = Trace<EventError>> where T: Send {
        let event = event.into();
        let inner = self.inner.clone();

        self.inner.pool.spawn_fn(move || {
            if let Some(listeners_lock) = try_throw!(inner.events.read()).get(&event) {
                let listeners = try_throw!(listeners_lock.read());

                // Don't bother if there aren't any listeners to invoke anyway
                if listeners.len() > 0 {
                    let mut listener_futures = Vec::with_capacity(listeners.len());

                    for listener_lock in listeners.iter() {
                        // Clone a local copy of the listener_lock that can be sent to the spawn
                        let listener_lock = listener_lock.clone();

                        // Clone a local copy of value that can be sent to the listener
                        let value = value.clone();

                        let listener_future = inner.pool.spawn_fn(move || -> EventResult<()> {
                            let mut cb_guard = try_throw!(listener_lock.cb.write());

                            // Use let binding to coerce value into Any
                            let opt: Option<Box<Any>> = Some(Box::new(value));

                            // Force a mutable reference to the callback
                            try_rethrow!((&mut *cb_guard)(opt.as_ref()));

                            Ok(())
                        });

                        listener_futures.push(listener_future);
                    }

                    // Join them all together into a single future and map the length of the results to the final future
                    return Ok(future::join_all(listener_futures)
                        .map(|executed: Vec<()>| executed.len()).boxed());
                }
            }

            Ok(futures::finished(0).boxed())
        }).flatten()
    }

    /// Variation of `emit_value` for `Sync` types, where intermediate copies are unnecessary.
    ///
    /// A copy of the value will be passed (by reference) to every listener.
    ///
    /// The `Future` returned by `emit` resolves to the number of listeners invoked,
    /// and any errors should be forward up.
    pub fn emit_value_sync<T: Any, E: Into<String>>(&mut self, event: E, value: T) -> impl Future<Item = usize, Error = Trace<EventError>> where T: Send + Sync {
        let event = event.into();
        let inner = self.inner.clone();

        self.inner.pool.spawn_fn(move || {
            if let Some(listeners_lock) = try_throw!(inner.events.read()).get(&event) {
                let listeners = try_throw!(listeners_lock.read());

                // Don't bother if there aren't any listeners to invoke anyway
                if listeners.len() > 0 {
                    let mut listener_futures = Vec::with_capacity(listeners.len());

                    // We know T is Send + Sync, and Box<Any> is really just Box<T>, so it is Send + Sync as well
                    #[derive(Clone)]
                    struct SyncWrapper {
                        inner: Arc<Option<Box<Any>>>
                    }

                    unsafe impl Send for SyncWrapper {}
                    unsafe impl Sync for SyncWrapper {}

                    // Use let binding to coerce value into Any
                    let wrapper = SyncWrapper { inner: Arc::new(Some(Box::new(value))) };

                    for listener_lock in listeners.iter() {
                        // Clone a local copy of the listener_lock that can be sent to the spawn
                        let listener_lock = listener_lock.clone();

                        let wrapper = wrapper.clone();

                        let listener_future = inner.pool.spawn_fn(move || -> EventResult<()> {
                            let mut cb_guard = try_throw!(listener_lock.cb.write());

                            // Force a mutable reference to the callback
                            try_rethrow!((&mut *cb_guard)((*wrapper.inner).as_ref()));

                            Ok(())
                        });

                        listener_futures.push(listener_future);
                    }

                    // Join them all together into a single future and map the length of the results to the final future
                    return Ok(future::join_all(listener_futures)
                        .map(|executed: Vec<()>| executed.len()).boxed());
                }
            }

            Ok(futures::finished(0).boxed())
        }).flatten()
    }
}