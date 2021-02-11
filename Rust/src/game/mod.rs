use bevy::prelude::App;
use gdnative::prelude::*;
use std::sync::{mpsc, Mutex};
mod time;
use time::Time;
mod gbevy;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Game {
    bevy: bevy::app::AppBuilder,
    time: Time,
}

#[methods]
impl Game {
    fn register(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "second_pass",
            args: &[],
        });
    }

    fn new(_owner: &Node) -> Self {
        let mut bevy = App::build();
        bevy.add_plugin(gbevy::GBevy);
        bevy.app.update();
        let time = Time::new();
        Game { bevy, time }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        gdnative::godot_print!("Hello from rust!");
    }

    #[export]
    fn _process(&mut self, owner: &Node, _delta: f64) {
        for _i in 0..self.time.process() {
            owner.emit_signal("second_pass", &[]);
            self.bevy_update(owner);
            self.b2g_update();
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

    fn bevy_update(&mut self, _owner: &Node) {
        self.bevy.app.update();
    }

    fn b2g_update(&self) {}
}

enum G2BMessage {}

struct Receiver(Mutex<mpsc::Receiver<G2BMessage>>);
impl std::ops::Deref for Receiver {
    type Target = Mutex<mpsc::Receiver<G2BMessage>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
