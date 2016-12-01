use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::{Deref, DerefMut};
use vec_map::VecMap;

use assimp;

use error::*;

use ::storage::generic::freelist::FreelistVecMap;
use super::mesh_cache::MeshCache;

pub type SourceScene<'a> = (assimp::Scene<'a>, String);

type Sourcelist<'a> = VecMap<Arc<SourceScene<'a>>>;

#[derive(Default)]
pub struct InnerSourceMap<'a> {
    pub sources: FreelistVecMap<Arc<SourceScene<'a>>>,
    pub mesh_cache: MeshCache<'a>
}

pub struct SourceMap<'a> {
    pub inner: RwLock<InnerSourceMap<'a>>
}

unsafe impl<'a> Send for SourceMap<'a> {}

unsafe impl<'a> Sync for SourceMap<'a> {}

impl<'a> SourceMap<'a> {
    pub fn new() -> SourceMap<'a> {
        SourceMap { inner: RwLock::default() }
    }

    pub fn mesh(&mut self, source_index: usize, mesh_index: usize) -> AppResult<Option<assimp::Mesh<'a>>> {
        //TODO: Handle poison errors
        let mut inner = self.inner.write().unwrap();

        if let Some(mesh) = inner.mesh_cache.get(source_index, mesh_index) {
            return Ok(Some(mesh.clone()));
        }

        let mesh = if let Some(source) = inner.sources.get(source_index) {
            source.0.mesh(mesh_index)
        } else { None };

        if let Some(mesh) = mesh {
            inner.mesh_cache.set(source_index, mesh_index, mesh.clone());

            return Ok(Some(mesh));
        }

        Ok(None)
    }

    pub fn get(&mut self, index: usize) -> AppResult<Option<Arc<SourceScene<'a>>>> {
        //TODO: Handle poison errors
        let inner = self.inner.read().unwrap();

        // Just clone the Arc instead of bothering with a reference to it
        Ok(inner.sources.get(index).map(|source| source.clone()))
    }

    pub fn add(&mut self, scene: Arc<assimp::Scene<'a>>, name: String) -> AppResult<usize> {
        let mut inner = self.inner.write().unwrap();

        // If we can't unwrap the scene Arc, then it's an invalid scene
        let raw_scene = try!(Arc::try_unwrap(scene).map_err(|_| AppError::InvalidScene));

        let index = inner.sources.add(Arc::new((raw_scene, name)));

        Ok(index)
    }

    pub fn remove(&mut self, index: usize) -> AppResult<Option<Arc<SourceScene<'a>>>> {
        let mut inner = self.inner.write().unwrap();

        Ok(inner.sources.remove(index))
    }
}
