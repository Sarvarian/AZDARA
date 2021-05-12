use gdnative::{core_types::GodotString, godot_print};
use legion::*;

#[system(for_each)]
pub fn test(_entity: &Entity) {}

#[system]
pub fn make_sprite(#[resource] srm: &super::srm::GodotServerResourceManager) {
    if let Err(err) = srm.make_sprite(GodotString::from_str("res://contents/icon.png")) {
        godot_print!("Error on make_sprite: {}", err);
    }
}
