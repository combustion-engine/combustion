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
        use ::components::scale::Component as Scale;
        use ::components::mesh::Component as Mesh;
        use ::components::gpu_buffer::Component as GPU_Buffer;
        use ::components::renderable::Component as Renderable;

        use ::game::components::turntable::Component as Turntable;

        let effects = assimp::postprocess::PostprocessEffectBuilder::target_realtime_fast()
            .optimize_meshes(true)
            .find_instances(true)
            .calc_tangent_space(true)
            .gen_smooth_normals(true)
            .improve_cache_locality(true);

        let buddha = try!(assimp::Scene::import("./models/twilight.obj", Some(effects.build())));

        let buddha_source = try!(sources.add(Arc::new(buddha), "Twilight".into()));

        info!("Twilight model stored at index: {}", buddha_source);

        let buffer = GPU_Buffer::new();

        world.create_now()
             .with(Transform::new())
             .with(Position(Point3::new(0.0, -0.25, 0.0)))
             .with(Rotation::none())
             .with(Scale::uniform(1.0 / 18.0))
             .with(Renderable::new())
             .with(Mesh::new(buddha_source, 0))
             .with(buffer.clone())
             .build();

        world.create_now()
             .with(Transform::new())
             .with(Position(Point3::new(-0.5, -0.25, 0.0)))
             .with(Rotation::none())
             .with(Scale::uniform(1.0 / 18.0))
             .with(Turntable { rate: 2.0 })
             .with(Renderable::new())
             .with(Mesh::new(buddha_source, 0))
             .with(buffer.clone())
             .build();

        world.create_now()
             .with(Transform::new())
             .with(Position(Point3::new(0.5, -0.25, 0.0)))
             .with(Rotation::none())
             .with(Scale::uniform(1.0 / 18.0))
             .with(Turntable { rate: -2.0 })
             .with(Renderable::new())
             .with(Mesh::new(buddha_source, 0))
             .with(buffer.clone())
             .build();

        Ok(())
    })
}