use crate::{rapier::RapierPhysics, res, sys};
use gdnative::prelude::*;
use legion::*;

type Owner = Node;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Game {
    world: legion::World,
    resources: legion::Resources,
    ready_schedule: legion::Schedule,
    process_schedule: legion::Schedule,
    physics_process_schedule: legion::Schedule,
}

#[methods]
impl Game {
    fn new(_owner: &Owner) -> Self {
        let (ready, process, physics_process) = sys::make_schedules();
        Game {
            world: make_world(),
            resources: make_resources(),
            ready_schedule: ready,
            process_schedule: process,
            physics_process_schedule: physics_process,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Owner) {
        if let Some(tree) = owner.get_tree() {
            unsafe {
                if let Some(root) = tree.assume_safe().root() {
                    let root = root.assume_safe();
                    root.set_usage(0);
                    if let Some(world) = root.find_world_2d() {
                        let canvas = world.assume_safe().canvas();
                        self.resources.insert(res::Canvas::new(canvas));
                    } else {
                        godot_error!("Getting World2D on Game Ready Failed!");
                    }
                } else {
                    godot_error!("Getting Viewport on Game Ready Failed!");
                }
            }
        }
        self.ready_schedule
            .execute(&mut self.world, &mut self.resources);
    }

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
        self.resources.get_mut_or_default::<RapierPhysics>().step();
    }

    fn set_delta_resource(&mut self, delta: f64) {
        let mut del = self.resources.get_mut_or_default::<res::Delta>();
        del.set(delta);
    }
}

pub fn make_world() -> World {
    let mut world = World::default();

    let palyer1 = (gdnative::core_types::Rid::new(),);
    world.extend(vec![palyer1]);

    world
}

pub fn make_resources() -> Resources {
    let mut res = Resources::default();

    res.insert(res::Input::default());
    res.insert(res::VisualServer::default());
    res.insert(res::ResourceLoader::default());
    res.insert(RapierPhysics::default());

    res
}
