use gdnative::prelude::*;
use legion::serialize::Canon;

mod com;
mod res;
mod sys;

type SerializeFilter = com::Player;

#[derive(NativeClass)]
#[inherit(Object)]
pub struct Game {
    game_name: GodotString,
    world: legion::World,
    resources: legion::Resources,
    process_schedule: legion::Schedule,
    physics_process_schedule: legion::Schedule,
    registry: legion::Registry<String>,
    godot_world: Ref<gdnative::api::World, gdnative::thread_access::Shared>,
}

#[methods]
impl Game {
    fn new(_owner: &Object) -> Self {
        let world = gdnative::api::World::new();
        let space = world.space();
        Game {
            game_name: GodotString::from_str("game"),
            world: sys::make_world(),
            resources: sys::make_resources(space),
            process_schedule: sys::make_process_schedule(),
            physics_process_schedule: sys::make_physics_process_schedule(),
            registry: sys::make_registry(),
            godot_world: world.into_shared(),
        }
    }

    #[export]
    fn ready(&mut self, _owner: &Object, game_name: GodotString) {
        self.game_name = game_name;
    }

    #[export]
    fn process(&mut self, _owner: &Object, delta: f64) {
        self.set_delta_resource(delta);
        self.process_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    #[export]
    fn physics_process(&mut self, _owner: &Object, delta: f64) {
        self.set_delta_resource(delta);
        self.physics_process_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    #[export]
    fn change_game_name(&mut self, _owner: &Object, game_name: GodotString) {
        self.game_name = game_name;
    }

    #[export]
    fn get_world(&self, _owner: &Object) -> Variant {
        Variant::from_object(&self.godot_world)
    }

    #[export]
    pub fn get_state(&self, _owner: &Object) -> Variant {
        let entity_serializer = Canon::default();
        if let Ok(json) = serde_json::to_value(&self.world.as_serializable(
            legion::component::<SerializeFilter>(),
            &self.registry,
            &entity_serializer,
        )) {
            Variant::from_str(json.to_string())
        } else {
            Variant::from_str("")
        }
    }

    #[export]
    pub fn save(&self, _owner: &Object) {
        let entity_serializer = Canon::default();
        if let Ok(save) = bincode::serialize(&self.world.as_serializable(
            legion::component::<SerializeFilter>(),
            &self.registry,
            &entity_serializer,
        )) {
            let file = gdnative::api::File::new();
            let save_path = GodotString::from_str(format!("user://{}", &self.game_name));
            match file.open(save_path.clone(), 2) {
                Err(err) => {
                    godot_error!("Game Save {} Failed, Because {}", save_path, err);
                }
                Ok(_) => {
                    file.store_buffer(TypedArray::from_vec(save));
                    godot_print!("Game Saved!")
                }
            }
            file.close();
        } else {
            godot_error!("bincode serialization failed")
        }
    }

    fn set_delta_resource(&mut self, delta: f64) {
        let mut del = self.resources.get_mut_or_default::<res::Delta>();
        &del.set(delta);
    }
}
