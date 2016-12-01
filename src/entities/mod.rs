use specs;

use error::*;

pub trait Entity<T> {
    fn new(world: &mut specs::World) -> AppResult<T>;

    fn raw(&self) -> specs::Entity;

    fn from_raw(specs::Entity) -> T;
}

pub mod camera;