#![feature(proc_macro)]

#[macro_use]
extern crate combustion_macros;

#[macro_use]
extern crate combustion_ecs as ecs;

#[derive(Component)]
pub struct SomeComponent {
    _field: i32
}
