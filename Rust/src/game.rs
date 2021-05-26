mod save_data;

use gdnative::prelude::*;

type Owner = Object;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Game {
    name: GodotString,
    _save_data: save_data::SaveData,
}

#[methods]
impl Game {
    fn new(_owner: &Owner) -> Self {
        Game {
            name: GodotString::from_str("Error Game Name Never Initialized"),
            _save_data: save_data::SaveData::default(),
        }
    }

    #[export]
    fn _init(&mut self, _owner: &Owner, name: GodotString) {
        godot_print!("Hello from godot! {}", name);
        self.name = name;
    }
}
