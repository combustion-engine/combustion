//! Effector component
//!
//! The idea of an Effector component is that it can be combined with the Node component to allow the parent
//! node to affect the children entities in some way, usually by propagating transforms to the children or similar.
//!
//! To allow an entity component to apply effects to a child of that entity,
//! this component takes a top-down approach for modifying child entities.
//! Therefore, the components that wish to affect child entities should implement the `Effector` trait.

use specs;

pub struct Component {
    pub ran: bool
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    pub fn new() -> Component {
        Component { ran: false }
    }
}

pub trait Effector<T> {
    fn effect(&self, child: &mut T);
}

/// Default implementation of `Effector` that does nothing.
impl<C> Effector<C> for C where C: specs::Component {
    #[inline(always)]
    default fn effect(&self, _: &mut C) {}
}