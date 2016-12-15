use glfw::Glfw;

use std::mem;
use std::ptr;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, PoisonError};

pub struct Requires {
    glfw: Arc<RwLock<Glfw>>,
}

impl Requires {
    pub fn extension(&self, extension: &'static str) -> Result<bool, PoisonError<RwLockReadGuard<Glfw>>> {
        Ok(self.glfw.read()?.extension_supported(extension))
    }
}