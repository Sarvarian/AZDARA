use gdnative::{godot_init, nativescript::InitHandle};

mod game;
mod log;
mod package_manager;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<log::Log>();
    handle.add_class::<log::AlertMessage>();
    handle.add_class::<package_manager::PackageManager>();
}

godot_init!(init);
