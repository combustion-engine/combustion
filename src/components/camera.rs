use specs;

use ::resources::projection::Resource as Projection;

pub use ::resources::projection::Kind;

/// The `Component` is really a `Projection` resource that can be applied to an entity to achieve dynamic cameras
pub type Component = Projection;

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}