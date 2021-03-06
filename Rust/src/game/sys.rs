use super::com::*;
use super::res::*;
use legion::*;

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer1 = (gdnative::core_types::Rid::new(),);
    world.extend(vec![palyer1]);

    world
}

pub fn make_resources(space: gdnative::core_types::Rid) -> Resources {
    let mut res = Resources::default();

    res.insert(super::srm::GodotServerResourceManager::new());
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
