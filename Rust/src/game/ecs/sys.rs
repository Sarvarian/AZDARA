use super::com::*;
use super::res::*;
use legion::*;
use uuid::Uuid;

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer1 = (
        Uuid::new_v4(),
        Player(0),
        DistrictId(1),
        Movable { x: 5u16, y: 5u16 },
    );
    let palyer2 = (
        Uuid::new_v4(),
        Player(1),
        DistrictId(1),
        Movable { x: 4u16, y: 5u16 },
    );

    let static1 = (Uuid::new_v4(), DistrictId(1), Static { x: 4, y: 8 });
    let static2 = (Uuid::new_v4(), DistrictId(1), Static { x: 5, y: 8 });

    world.extend(vec![palyer1, palyer2]);
    world.extend(vec![static1, static2]);

    world
}

pub fn make_resources() -> Resources {
    let mut res = Resources::default();

    res.insert(Nav2D::new());

    res
}

pub fn make_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

pub fn make_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();

    registry.register::<Uuid>("uuid".to_string());
    registry.register::<DistrictId>("district_id".to_string());
    registry.register::<Player>("player".to_string());
    registry.register::<Movable>("movable".to_string());
    registry.register::<Static>("static".to_string());

    registry
}

#[system(for_each)]
pub fn test(_entity: &Entity) {}
