use gdnative::{api::Node, methods, NativeClass};
mod gbevy;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Game {
    handle: Option<std::thread::JoinHandle<()>>,
    sender: Option<std::sync::mpsc::Sender<gbevy::G2BMessage>>,
}

#[methods]
impl Game {
    fn new(_owner: &Node) -> Self {
        Game {
            handle: None,
            sender: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        let viewport;
        unsafe {
            viewport = owner
                .get_tree()
                .unwrap()
                .assume_safe() // unsafe
                .root()
                .unwrap()
                .assume_safe() // unsafe
                .get_viewport_rid();
        }

        let (tx, receiver) = std::sync::mpsc::channel();
        self.sender = Some(tx);
        let setup = gbevy::Setup { receiver, viewport };
        self.handle = Some(std::thread::spawn(move || {
            gbevy::bevy(setup);
        }));
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {
        if let Some(sender) = self.sender.take() {
            sender.send(gbevy::G2BMessage::Quit).unwrap_or(());
        };
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap_or(());
        }
    }
}
