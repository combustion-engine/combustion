use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::{Deref, DerefMut};

use num_cpus;
use specs;

use error::*;

use storage::generic::freelist::FreelistVecMap;
pub use ::scene::sourcemap::SourceMap;
pub use ::scene::graph::SceneGraph;

use resources;
use entities::camera::Entity as Camera;
use systems;

use entities::Entity;

pub struct Scene<'a> {
    pub planner: specs::Planner<systems::Delta>,
    pub sources: SourceMap<'a>,
}

impl<'a> Scene<'a> {
    pub fn new() -> AppResult<Scene<'a>> {
        //Initialize world and create planner
        let mut planner = {
            let mut world = specs::World::new();

            //Register all regular and game components
            ::components::register_all(&mut world);
            ::game::components::register_all(&mut world);

            //Cursor position helper resource
            world.add_resource(resources::cursor::Resource::new());

            //Event queue resource
            world.add_resource(resources::event_queue::Resource::new());

            //Render queue resource
            world.add_resource(resources::render_queue::Resource::new());

            let camera = try!(Camera::new(&mut world));
            world.add_resource::<resources::camera::Resource>(camera.into());

            let graph = SceneGraph::new(&world);
            world.add_resource::<resources::scene_graph::Resource>(graph.into());

            specs::Planner::new(world, num_cpus::get())
        };

        planner.add_system(systems::constraints::System, "ConstrainSystem",
                           systems::Priorities::Constraints as specs::Priority);

        planner.add_system(systems::transform::System, "TransformSystem",
                           systems::Priorities::Transforms as specs::Priority);

        ::game::scene::add_systems(&mut planner);

        planner.dispatch(0.0);
        planner.wait();

        Ok(Scene {
            planner: planner,
            sources: SourceMap::new(),
        })
    }

    #[inline]
    pub fn camera(&mut self) -> RwLockReadGuard<resources::camera::Resource> {
        self.planner.mut_world().read_resource()
    }

    #[inline]
    pub fn mut_camera(&mut self) -> RwLockWriteGuard<resources::camera::Resource> {
        self.planner.mut_world().write_resource()
    }

    #[inline(always)]
    pub fn world(&mut self) -> &mut specs::World {
        self.planner.mut_world()
    }

    #[inline]
    pub fn with_world<F, U>(&mut self, mut f: F) -> U where F: FnMut(&mut specs::World) -> U {
        let mut world = self.planner.mut_world();

        f(&mut world)
    }

    #[inline]
    pub fn with_world_sources<F, U>(&mut self, mut f: F) -> U where F: FnMut(&mut specs::World, &mut SourceMap<'a>) -> U {
        let mut world = self.planner.mut_world();

        f(&mut world, &mut self.sources)
    }

    #[inline(always)]
    pub fn update(&mut self, delta: systems::Delta) {
        self.planner.dispatch(delta);
    }

    #[inline(always)]
    pub fn wait(&mut self) {
        self.planner.wait();
    }
}