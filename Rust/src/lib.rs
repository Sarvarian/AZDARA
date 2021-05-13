use gdnative::{godot_init, nativescript::InitHandle};

mod com;
mod game;
mod rapier;
mod res;
mod sys;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

godot_init!(init);
