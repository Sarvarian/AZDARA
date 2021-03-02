use gdnative::prelude::*;

mod com;
mod res;
mod sys;

pub struct ECSHandle {
    world: legion::World,
    resources: legion::Resources,
    schedule: legion::Schedule,
    registry: legion::Registry<String>,
}

impl ECSHandle {
    pub fn new() -> Self {
        ECSHandle {
            world: sys::make_world(),
            resources: sys::make_resources(),
            schedule: sys::make_schedule(),
            registry: sys::make_registry(),
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
                default: Variant::from_vector2(&Vector2::new(5f32, 5f32)),
                export_info: ExportInfo::new(VariantType::Vector2),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    pub fn get_state(&self) -> Variant {
        if let Ok(json) = serde_json::to_value(
            &self
                .world
                .as_serializable(legion::component::<uuid::Uuid>(), &self.registry),
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
