use gdnative::prelude::*;
use legion::serialize::Canon;
use serde::de::DeserializeSeed;

mod com;
mod res;
mod srm;
mod sys;

type SerializeFilter = com::Player;

#[derive(NativeClass)]
#[inherit(Object)]
pub struct Game {
    world: legion::World,
    resources: legion::Resources,
    ready_schedule: legion::Schedule,
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
            world: sys::make_world(),
            resources: sys::make_resources(space),
            ready_schedule: sys::make_ready_schedule(),
            process_schedule: sys::make_process_schedule(),
            physics_process_schedule: sys::make_physics_process_schedule(),
            registry: sys::make_registry(),
            godot_world: world.into_shared(),
        }
    }

    #[export]
    fn ready(&mut self, _owner: &Object) {
        self.ready_schedule
            .execute(&mut self.world, &mut self.resources);
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
    pub fn save_game(&self, _owner: &Object, game_name: GodotString) {
        let entity_serializer = Canon::default();
        if let Ok(save) = bincode::serialize(&self.world.as_serializable(
            legion::component::<SerializeFilter>(),
            &self.registry,
            &entity_serializer,
        )) {
            let file = gdnative::api::File::new();
            let save_path = GodotString::from_str(format!("user://{}", game_name.to_lowercase()));
            match file.open(save_path.clone(), 2) {
                Err(err) => {
                    godot_error!("Game Save {} Failed, Because {}", save_path, err);
                }
                Ok(_) => {
                    godot_print!("{:?}", save);
                    file.store_buffer(TypedArray::from_vec(save));
                    godot_print!("Game Saved Successfully!")
                }
            }
            file.close();
        } else {
            godot_error!("bincode serialization failed")
        }
    }

    #[export]
    pub fn load_game(&mut self, _owner: &Object, game_name: GodotString) {
        let file = gdnative::api::File::new();
        let load_path = GodotString::from_str(format!("user://{}", game_name.to_lowercase()));
        match file.open(load_path.clone(), 1) {
            Err(err) => {
                godot_error!("Game Load {} Failed, Because {}", load_path, err);
            }
            Ok(_) => {
                let load = file.get_buffer(file.get_len());
                let load = (0..load.len()).fold(
                    Vec::<u8>::with_capacity(load.len() as usize),
                    |mut vec, i| {
                        vec.push(load.get(i));
                        vec
                    },
                );
                godot_print!("{:?}", load);
                let entity_serializer = Canon::default();
                let mut des =
                    bincode::Deserializer::from_slice(load.as_slice(), bincode::options());
                match self
                    .registry
                    .as_deserialize(&entity_serializer)
                    .deserialize(&mut des)
                {
                    Ok(load) => {
                        self.world = load;
                        self.ready_schedule
                            .execute(&mut self.world, &mut self.resources);
                        godot_print!("Game Loaded Successfully!");
                    }
                    Err(err) => {
                        godot_error!(
                            "Deserializing Load {} Failed, Because of {}",
                            load_path,
                            err
                        );
                    }
                }
            }
        }
        file.close();
    }

    fn set_delta_resource(&mut self, delta: f64) {
        let mut del = self.resources.get_mut_or_default::<res::Delta>();
        &del.set(delta);
    }
}
