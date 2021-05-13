use input::InputSystems;
use legion::*;
use movement::MovementSystems;

mod input;
mod movement;
mod ready;

pub fn make_ready_schedule() -> Schedule {
    Schedule::builder()
        .add_system(ready::make_sprite_system())
        .build()
}

pub fn make_process_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

pub fn make_physics_process_schedule() -> Schedule {
    Schedule::builder()
        .add_system(physics_system())
        .add_input_systems()
        .add_movement_systems()
        .build()
}

#[system(for_each)]
fn test(_entity: &Entity) {}

#[system(for_each)]
fn physics(_entity: &Entity) {}
