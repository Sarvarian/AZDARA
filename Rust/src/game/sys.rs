use legion::*;

pub fn make_process_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

pub fn make_physics_process_schedule() -> Schedule {
    Schedule::builder().add_system(test_system()).build()
}

#[system(for_each)]
pub fn test(_entity: &Entity) {}
