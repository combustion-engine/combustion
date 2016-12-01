use specs;

use entities::Entity;
use entities::camera::Entity as Camera;

pub struct Resource(pub Camera);

impl Resource {
    #[inline]
    pub fn new(camera: Camera) -> Resource {
        Resource(camera)
    }

    #[inline(always)]
    pub fn entity(&self) -> specs::Entity {
        self.0.raw()
    }
}