use gdnative::{godot_init, nativescript::InitHandle};

mod game;
mod helpers;
mod test;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<test::Test>();
}

godot_init!(init);
