//! Game scene setup

use specs;

use super::systems;

pub fn add_systems(mut planner: &mut specs::Planner<systems::Delta>) {
    planner.add_system(systems::turntable::System, "TurntableSystem",
                       systems::Priorities::Turntable as specs::Priority);

    planner.add_system(systems::blackhole::System, "BlackholeSystem",
                       systems::Priorities::Blackhole as specs::Priority);

    planner.add_system(systems::bob::System, "BobSystem", systems::Priorities::Bob as specs::Priority);
}