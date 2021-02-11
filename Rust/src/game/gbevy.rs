use bevy::{
    ecs::{IntoSystem, World},
    prelude::{AppBuilder, Plugin},
};
use euclid::*;
use gdnative::prelude::*;

mod com;
mod sys;

pub struct GBevy;

impl Plugin for GBevy {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(sys::setup.system());
    }
}

pub fn register(builder: &ClassBuilder<super::Game>) {
    builder.add_signal(Signal {
        name: "spawn_player",
        args: &[SignalArgument {
            name: "position",
            default: Variant::from_vector2(&Vector2D::new(5f32, 5f32)),
            export_info: ExportInfo::new(VariantType::Vector2),
            usage: PropertyUsage::DEFAULT,
        }],
    });
}

pub fn get_state(world: &World) -> Variant {
    let dictionary = Dictionary::new();
    for (_p, pos) in world.query::<(&com::Player, &com::Movable)>() {
        dictionary.insert(
            "Player",
            Variant::from_vector2(&Vector2D::new(pos.x as f32, pos.y as f32)),
        )
    }
    Variant::from_dictionary(&dictionary.into_shared())
}

pub fn update(world: &World, owner: &Node) {}
