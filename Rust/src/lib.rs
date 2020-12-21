use gdnative::{godot_init, nativescript::InitHandle};
mod game;
mod gbevy;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

godot_init!(init);
