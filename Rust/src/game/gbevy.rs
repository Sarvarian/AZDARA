use bevy::prelude::*;
use std::sync::{mpsc, Mutex};
mod receive_handler;
mod server_test;

pub fn bevy(setup: Setup) {
    let (receiver, viewport) = (setup.receiver, setup.viewport);
    App::build()
        .add_plugins(MinimalPlugins)
        .add_resource(Receiver(Mutex::new(receiver)))
        .add_resource(Viewport(viewport))
        .add_plugin(receive_handler::ReceiveHandler)
        .add_plugin(server_test::ServerTest)
        .run();
}

pub struct Setup {
    pub receiver: mpsc::Receiver<G2BMessage>,
    pub viewport: gdnative::prelude::Rid,
}

pub enum G2BMessage {
    Quit,
}

struct Viewport(gdnative::prelude::Rid);
impl std::ops::Deref for Viewport {
    type Target = gdnative::prelude::Rid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Receiver(Mutex<mpsc::Receiver<G2BMessage>>);
impl std::ops::Deref for Receiver {
    type Target = Mutex<mpsc::Receiver<G2BMessage>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
