use gdnative::{api::Node, methods, NativeClass};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Character {
    #[property(default = 5)]
    hp: u8,
}

#[methods]
impl Character {
    fn new(_owner: &Node) -> Self {
        Character { hp: 5 }
    }

    #[export]
    fn _enter_tree(&mut self, owner: &Node) {
        owner.add_to_group("Characters", true);
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {}
}
