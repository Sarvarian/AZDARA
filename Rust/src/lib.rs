use gdnative::{godot_init, nativescript::InitHandle};

mod game;
mod log;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<log::Log>();
}

godot_init!(init);
