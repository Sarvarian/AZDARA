use gdnative::{godot_init, nativescript::InitHandle};
mod aban;

fn init(handle: InitHandle) {
    handle.add_class::<aban::Aban>();
}

godot_init!(init);
