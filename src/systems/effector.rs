use specs;
use specs::Join;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use ::components::effector::Effector;
use ::components::effector::Component as EffectorNode;
use ::components::transform::Component as Transform;

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, _: specs::RunArg, _: super::Delta) {

    }
}