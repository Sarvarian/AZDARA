use bevy::prelude::App;
use gdnative::prelude::*;
use std::sync::{mpsc, Mutex};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Aban {
    games: Vec<Game>,
}

#[methods]
impl Aban {
    fn new(_owner: &Node) -> Self {
        Aban {
            games: Vec::with_capacity(1),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        gdnative::godot_print!("Hello from rust!");
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {}

    #[export]
    fn new_game(&mut self, _owner: &Node) -> usize {
        self.games.push(Game::new());
        self.games.len()
    }
}

struct Game {
    sender: std::sync::mpsc::Sender<G2BMessage>,
    bevy: bevy::app::AppBuilder,
}

impl Game {
    fn new() -> Self {
        let (tx, receiver) = std::sync::mpsc::channel();
        let mut app = App::build();
        app.add_resource(Receiver(Mutex::new(receiver)));
        app.app.update();
        Game {
            sender: tx,
            bevy: app,
        }
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
