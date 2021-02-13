use ecs::ECSHandle;
use gdnative::prelude::*;
use std::sync::{mpsc, Mutex};
use time::Time;

mod ecs;
mod time;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Game {
    time: Time,
    ecs_handle: ECSHandle,
}

#[methods]
impl Game {
    fn register(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "second_pass",
            args: &[],
        });
        ECSHandle::register(builder)
    }

    fn new(_owner: &Node) -> Self {
        let time = Time::new();
        let ecs_handle = ECSHandle::new();
        Game { time, ecs_handle }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        gdnative::godot_print!("Hello from rust!");
    }

    #[export]
    fn _process(&mut self, owner: &Node, _delta: f64) {
        for _i in 0..self.time.process() {
            owner.emit_signal("second_pass", &[]);
            self.ecs_handle.update(owner);
        }
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {}

    #[export]
    fn set_game_speed(&mut self, _owner: &Node, mut speed: f64) {
        if !(speed > 0f64) {
            speed = 0.001;
        }
        self.time.speed = speed;
    }

    #[export]
    fn state(&self, _owner: &Node) -> Variant {
        self.ecs_handle.get_state()
    }
}

enum G2BMessage {}

struct Receiver(Mutex<mpsc::Receiver<G2BMessage>>);
impl std::ops::Deref for Receiver {
    type Target = Mutex<mpsc::Receiver<G2BMessage>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
