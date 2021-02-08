use gdnative::{godot_init, nativescript::InitHandle};
mod character;
mod game;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<character::Character>();
}

godot_init!(init);
