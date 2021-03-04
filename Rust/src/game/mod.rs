use ecs::ECSHandle;
use gdnative::prelude::*;

mod ecs;

#[derive(NativeClass)]
#[inherit(Object)]
pub struct Game {
    ecs_handle: ECSHandle,
    world: Ref<gdnative::api::World, gdnative::thread_access::Shared>,
}

#[methods]
impl Game {
    fn new(_owner: &Object) -> Self {
        let world = gdnative::api::World::new();
        let space = world.space();
        Game {
            ecs_handle: ECSHandle::new(space),
            world: world.into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Object) {
        gdnative::godot_print!("Hello from rust!");
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Object, _delta: f64) {
        self.ecs_handle.update();
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Object) {}

    #[export]
    fn get_world(&self, _owner: &Object) -> Variant {
        Variant::from_object(&self.world)
    }
}
