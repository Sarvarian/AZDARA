use bevy::prelude::App;
use gdnative::prelude::*;
use std::sync::{mpsc, Mutex};
mod time;
use time::Time;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Game {
    sender: std::sync::mpsc::Sender<G2BMessage>,
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
        builder.add_signal(Signal {
            name: "minute_pass",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hour_pass",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "day_pass",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "month_pass",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "year_pass",
            args: &[],
        });
    }

    fn new(_owner: &Node) -> Self {
        let (tx, receiver) = std::sync::mpsc::channel();
        let mut bevy = App::build();
        bevy.add_resource(Receiver(Mutex::new(receiver)));
        bevy.app.update();
        let time = Time::new();
        Game {
            sender: tx,
            bevy,
            time,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        gdnative::godot_print!("Hello from rust!");
    }

    #[export]
    fn _process(&mut self, owner: &Node, delta: f64) {
        self.time.process(owner, delta);
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {}

    #[export]
    fn set_game_speed(&mut self, _owner: &Node, speed: f64) {
        self.time.speed = speed;
        // if let Some(speed) = speed.try_to_f64() {
        // }
    }

    #[export]
    fn get_timedate(&self, _owner: &Node) -> Variant {
        Variant::from_byte_array(&TypedArray::from_vec(self.time.get().to_vec()))
    }

    #[export]
    fn skip_time_a_second(&mut self, owner: &Node) {
        self.time.add_second(owner)
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
