use input::InputSystems;
use legion::{systems::Builder, *};
use movement::MovementSystems;

mod input;
mod movement;
mod physics;
mod ready;

struct Builder3 {
    ready: Builder,
    process: Builder,
    physics_process: Builder,
}

impl std::convert::Into<(Schedule, Schedule, Schedule)> for Builder3 {
    fn into(mut self) -> (Schedule, Schedule, Schedule) {
        (
            self.ready.build(),
            self.process.build(),
            self.physics_process.build(),
        )
    }
}

pub fn make_schedules() -> (Schedule, Schedule, Schedule) {
    let mut builders = Builder3 {
        ready: Schedule::builder(),
        process: Schedule::builder(),
        physics_process: Schedule::builder(),
    };

    builders.ready.add_system(ready::make_sprite_system());

    builders.process.add_system(test_system());

    builders.physics_process.add_system(physics_system());

    builders.add_input_systems().add_movement_systems();

    builders.into()
}

#[system(for_each)]
fn test(_entity: &Entity) {}

#[system(for_each)]
fn physics(_entity: &Entity) {}
