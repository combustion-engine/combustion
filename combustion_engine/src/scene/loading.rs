use std::sync::Arc;
use std::path::{Path, PathBuf};

use specs;
use nalgebra::*;
use assimp;

use error::*;

use resources;
use systems;

use super::Scene;

pub trait SceneExtLoader {
    fn load_scene<P: AsRef<Path>>(&mut self, node: (), path: P) -> AppResult<()>;
}

impl<'a> SceneExtLoader for Scene<'a> {
    fn load_scene<P: AsRef<Path>>(&mut self, _: (), _: P) -> AppResult<()> {
        Ok(())
    }
}