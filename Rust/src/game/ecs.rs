use gdnative::prelude::*;

mod com;
mod res;
mod sys;

pub struct ECSHandle {
    world: legion::World,
    resources: legion::Resources,
    schedule: legion::Schedule,
    // registry: legion::Registry<String>,
}

impl ECSHandle {
    pub fn new(space: Rid) -> Self {
        ECSHandle {
            world: sys::make_world(),
            resources: sys::make_resources(space),
            schedule: sys::make_schedule(),
            // registry: sys::make_registry(),
        }
    }

    // pub fn get_state(&self) -> Variant {
    //     if let Ok(json) = serde_json::to_value(&self.world.as_serializable(
    //         legion::component::<gdnative::core_types::Rid>(),
    //         &self.registry,
    //     )) {
    //         Variant::from_str(json.to_string())
    //     } else {
    //         Variant::from_str("serde_jason Failed!".to_string())
    //     }
    // }

    pub fn update(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}
