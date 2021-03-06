use std::sync::mpsc::Sender;

use super::com::*;
use super::msg::*;
use super::res::*;
use legion::*;

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer1 = (gdnative::core_types::Rid::new(),);
    world.extend(vec![palyer1]);

    world
}

pub fn make_resources(
    space: gdnative::core_types::Rid,
    process_message_sender: Sender<ECSMessages>,
    physics_process_message_sender: Sender<ECSMessages>,
) -> Resources {
    let mut res = Resources::default();

    res.insert(super::srm::GodotServerResourceManager::new());
    res.insert(ProcessMessageSender::new(process_message_sender));
    res.insert(PhysicsProcessMessageSender::new(
        physics_process_message_sender,
    ));
    res.insert(Input::new());
    res.insert(WorldSpace::new(space));

    res
}

pub fn make_ready_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

pub fn make_process_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

pub fn make_physics_process_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

pub fn make_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();

    registry.register::<Position>("pos".to_string());
    registry.register::<Player>("player".to_string());

    registry
}

#[system(for_each)]
pub fn test(_entity: &Entity) {}
