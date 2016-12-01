use glfw::{self, Context, WindowEvent};
use std::sync::mpsc;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::Duration as StdDuration;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use vec_map::VecMap;

use time::{Duration, PreciseTime};
use nalgebra::{Point3, Vector3};

use num_cpus;
use specs;
use specs::Join;

use assimp;

use error::*;

use resources;
use entities::camera::Entity as Camera;
use systems;

use entities::Entity;

use storage::generic::freelist::FreelistVecMap;

//TODO: Replace with with a LRU cache variant
pub struct MeshCache<'a> {
    pub cache: VecMap<VecMap<assimp::Mesh<'a>>>
}

impl<'a> Default for MeshCache<'a> {
    #[inline(always)]
    fn default() -> MeshCache<'a> {
        MeshCache {
            cache: Default::default()
        }
    }
}

impl<'a> MeshCache<'a> {
    pub fn get(&self, x: usize, y: usize) -> Option<&assimp::Mesh<'a>> {
        if let Some(ycache) = self.cache.get(x) {
            ycache.get(y)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, mesh: assimp::Mesh<'a>) {
        if let Some(mut ycache) = self.cache.get_mut(x) {
            ycache.insert(y, mesh);

            return;
        }

        let mut ycache = VecMap::new();

        ycache.insert(y, mesh);

        self.cache.insert(x, ycache);
    }
}
