use ecs::ECSHandle;
use gdnative::prelude::*;

mod ecs;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Game {
    ecs_handle: ECSHandle,
    world: Ref<gdnative::api::World, gdnative::thread_access::Shared>,
    space: Rid,
}

#[methods]
impl Game {
    fn new(_owner: &Node) -> Self {
        let world = gdnative::api::World::new();
        let space = world.space();
        Game {
            ecs_handle: ECSHandle::new(),
            world: world.into_shared(),
            space,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        gdnative::godot_print!("Hello from rust!");
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node, _delta: f64) {
        self.ecs_handle.update(owner);
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {}

    #[export]
    fn get_world(&self, _owner: &Node) -> Variant {
        Variant::from_object(&self.world)
    }
}
