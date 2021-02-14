use euclid::*;
use gdnative::prelude::*;

mod com;
mod sys;

pub struct ECSHandle {
    world: legion::World,
    resources: legion::Resources,
    schedule: legion::Schedule,
    registry: legion::Registry<String>,
}

impl ECSHandle {
    pub fn new() -> Self {
        let world = sys::make_world();
        let resources = sys::make_resources();
        let schedule = sys::make_schedule();
        let registry = sys::make_registry();
        ECSHandle {
            world,
            resources,
            schedule,
            registry,
        }
    }

    pub fn register(builder: &ClassBuilder<super::Game>) {
        builder.add_signal(Signal {
            name: "game_updated",
            args: &[],
        });
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
        // Variant::from_dictionary(&Dictionary::new().into_shared())

        if let Ok(json) = serde_json::to_value(
            &self
                .world
                .as_serializable(legion::component::<com::DistrictId>(), &self.registry),
        ) {
            Variant::from_str(json.to_string())
        } else {
            Variant::from_str("serde_jason Failed!".to_string())
        }
    }

    pub fn update(&mut self, owner: &Node) {
        self.schedule.execute(&mut self.world, &mut self.resources);
        owner.emit_signal("game_updated", &[]);
    }
}
