use specs;

pub mod scene_graph;
pub mod cursor;
pub mod camera;
pub mod event_queue;
pub mod render_queue;
pub mod projection;

pub fn add_defaults(mut world: &mut specs::World) {
    let scene_graph = scene_graph::Resource::new(world);

    world.add_resource(scene_graph);
    world.add_resource(cursor::Resource::new());
    world.add_resource(event_queue::Resource::new());
    world.add_resource(render_queue::Resource::new());
}