use gdnative::prelude::*;

use legion::*;

mod com;
mod res;
mod srm;
mod sys;

type Owner = Node;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Game {
    world: legion::World,
    resources: legion::Resources,
    process_schedule: legion::Schedule,
    physics_process_schedule: legion::Schedule,
}

#[methods]
impl Game {
    fn new(_owner: &Owner) -> Self {
        let world = gdnative::api::World2D::new();
        let canvas = world.canvas();
        Game {
            world: make_world(),
            resources: make_resources(canvas),
            process_schedule: make_process_schedule(),
            physics_process_schedule: make_physics_process_schedule(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Owner) {}

    #[export]
    fn _process(&mut self, _owner: &Owner, delta: f64) {
        self.set_delta_resource(delta);
        self.process_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Owner, delta: f64) {
        self.set_delta_resource(delta);
        self.physics_process_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    fn set_delta_resource(&mut self, delta: f64) {
        let mut del = self.resources.get_mut_or_default::<res::Delta>();
        &del.set(delta);
    }
}

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer1 = (gdnative::core_types::Rid::new(),);
    world.extend(vec![palyer1]);

    world
}

pub fn make_resources(canvas: gdnative::core_types::Rid) -> Resources {
    let mut res = Resources::default();

    res.insert(srm::GodotServerResourceManager::new(canvas));
    res.insert(res::Input::new());

    res
}

pub fn make_process_schedule() -> Schedule {
    Schedule::builder().add_system(sys::test_system()).build()
}

pub fn make_physics_process_schedule() -> Schedule {
    Schedule::builder().add_system(sys::test_system()).build()
}
