use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread::{self, Thread};

pub mod error;

pub use self::error::*;

pub enum SchedulerEvent {
    Bootstrap(Thread),
}

pub struct Scheduler {
    render_thread: Thread,
    ecs_thread: Thread,
    event_thread: Thread,
    scheduler_thread: Thread,
}

impl Scheduler {
    pub fn new() -> SchedulerResult<Arc<Scheduler>> {
        let (bootstrap_tx, bootstrap_rx) = mpsc::sync_channel();

        let scheduler_join_handle = thread::Builder::new().name("Scheduler").spawn(move || {
            let scheduler = bootstrap_rx.recv().unwrap();

            // Do scheduling stuff
        })?;



        Ok(())
    }
}