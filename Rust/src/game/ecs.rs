use euclid::*;
use gdnative::prelude::*;
use legion::{Schedule, World};

mod com;
mod sys;

pub struct ECSHandle {
    world: World,
    schedule: Schedule,
}

impl ECSHandle {
    pub fn new() -> Self {
        let world = sys::make_world();
        let schedule = sys::make_schedule();
        ECSHandle { world, schedule }
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

    pub fn get_state(&self) -> Variant {
        //     let dictionary = Dictionary::new();
        //     for (_p, pos) in world.query::<(&com::Player, &com::Movable)>() {
        //         dictionary.insert(
        //             "Player",
        //             Variant::from_vector2(&Vector2D::new(pos.x as f32, pos.y as f32)),
        //         )
        //     }
        Variant::from_dictionary(&Dictionary::new().into_shared())
    }

    pub fn update(&mut self, owner: &Node) {}
}
