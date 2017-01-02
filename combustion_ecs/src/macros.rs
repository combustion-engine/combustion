#[macro_export]
macro_rules! ecs_register_mod {
    ($world:expr, $component_mod:ident) => { $world.register::<$component_mod::Component>() }
}
