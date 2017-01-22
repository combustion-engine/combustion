#[macro_use]
extern crate combustion_macros;

#[macro_use]
extern crate combustion_ecs;

#[derive(Component, Debug)]
#[ecs(path = "combustion_ecs", storage = "HashMapStorage")]
pub struct SomeComponent {
    _field: i32
}