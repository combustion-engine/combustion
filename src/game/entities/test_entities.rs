use std::sync::Arc;

use specs;
use nalgebra::*;
use assimp;

use error::*;

use resources;
use systems;

use scene::Scene;

pub fn load(mut scene: &mut Scene) -> AppResult<()> {
    scene.with_world_sources(|mut world: &mut specs::World, mut sources| -> AppResult<()> {
        use ::components::transform::Component as Transform;
        use ::components::position::Component as Position;
        use ::components::rotation::Component as Rotation;
        use ::components::mesh::Component as Mesh;
        use ::components::gpu_buffer::Component as GPU_Buffer;
        use ::components::renderable::Component as Renderable;

        use ::game::components::turntable::Component as Turntable;

        let buddha = try!(assimp::Scene::import("./models/buddha_uv.fbx", None));

        let buddha_source = try!(sources.add(Arc::new(buddha), "Buddha".into()));

        println!("Buddha model stored at index: {}", buddha_source);

        world.create_now()
             .with(Transform::new())
             .with(Position(Point3::new(0.0, 0.0, 0.0)))
             .with(Rotation::none())
             .with(Renderable::new())
             .with(Mesh::new(buddha_source, 0))
             .with(GPU_Buffer::new())
             .build();

        world.create_now()
             .with(Transform::new())
             .with(Position(Point3::new(-0.5, 0.0, 0.0)))
             .with(Rotation::none())
             .with(Turntable { rate: 2.0 })
             .with(Renderable::new())
             .with(Mesh::new(buddha_source, 0))
             .with(GPU_Buffer::new())
             .build();

        world.create_now()
             .with(Transform::new())
             .with(Position(Point3::new(0.5, 0.0, 0.0)))
             .with(Rotation::none())
             .with(Turntable { rate: -2.0 })
             .with(Renderable::new())
             .with(Mesh::new(buddha_source, 0))
             .with(GPU_Buffer::new())
             .build();

        Ok(())
    })
}