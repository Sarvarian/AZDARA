use gdnative::{api::Node, godot_print, methods, NativeClass};
mod time;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Game {
    time: time::Time,
}

#[methods]
impl Game {
    fn new(_owner: &Node) -> Self {
        Game {
            time: time::Time::new(),
        }
    }

    #[export]
    fn _enter_tree(&mut self, _owner: &Node) {}

    #[export]
    fn _ready(&mut self, _owner: &Node) {}

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f64) {
        self.time.process(delta);
    }
    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {}

    #[export]
    pub fn godod_print_timedate(&self, _owner: &Node) {
        godot_print!(
            "{}/{}/{} {}:{}:{}",
            self.time.timedate.year,
            self.time.timedate.month,
            self.time.timedate.day,
            self.time.timedate.hour,
            self.time.timedate.minute,
            self.time.timedate.second,
        );
    }
}
