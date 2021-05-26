use gdnative::prelude::*;

type Owner = Object;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Game {}

#[methods]
impl Game {
    fn new(_owner: &Owner) -> Self {
        Game {}
    }

    #[export]
    fn _ready(&mut self, _owner: &Owner) {}

    #[export]
    fn _process(&mut self, _owner: &Owner, _delta: f64) {}

    #[export]
    fn _physics_process(&mut self, _owner: &Owner, _delta: f64) {}
}
