use gdnative::prelude::*;

type Owner = Object;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Game {
    name: GodotString,
}

#[methods]
impl Game {
    fn new(_owner: &Owner) -> Self {
        Game {
            name: GodotString::from_str("Error name dosnt initialize"),
        }
    }

    #[export]
    fn _init(&mut self, _owner: &Owner, name: GodotString) {
        godot_print!("Hello from godot! {}", name);
        self.name = name;
    }
}
