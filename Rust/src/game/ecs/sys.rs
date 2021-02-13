use super::com::*;
use legion::*;

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer = (Player(0), DistrictId(1), Movable { x: 5u16, y: 5u16 });

    world.extend(vec![palyer]);

    world
}

pub fn make_schedule() -> Schedule {
    Schedule::builder().build()
}

pub fn make_resources() -> Resources {
    Resources::default()
}

#[system(for_each)]
pub fn test(entity: &Entity) {}
