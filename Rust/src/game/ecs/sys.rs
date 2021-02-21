use super::com::*;
use super::res::*;
use legion::*;

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer = (Player(0), DistrictId(1), Movable { x: 5u16, y: 5u16 });

    world.extend(vec![palyer]);

    world
}

pub fn make_resources() -> Resources {
    let mut res = Resources::default();

    res.insert(Nav2D::new());

    res
}

pub fn make_schedule() -> Schedule {
    Schedule::builder().build()
}

pub fn make_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();

    registry.register::<DistrictId>("district_id".to_string());
    registry.register::<Player>("player".to_string());
    registry.register::<Movable>("movable".to_string());
    registry.register::<Static>("static".to_string());

    registry
}

#[system(for_each)]
pub fn test(_entity: &Entity) {}
