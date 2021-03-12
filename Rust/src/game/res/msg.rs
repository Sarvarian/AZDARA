use super::super::msg::*;
use std::sync::mpsc::Sender;

pub struct ProcessMessageSender(Sender<FromECSMessages>);

impl std::ops::Deref for ProcessMessageSender {
    type Target = Sender<FromECSMessages>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProcessMessageSender {
    pub fn new(process_message_sender: Sender<FromECSMessages>) -> Self {
        ProcessMessageSender(process_message_sender)
    }
}

pub struct PhysicsProcessMessageSender(Sender<FromECSMessages>);

impl std::ops::Deref for PhysicsProcessMessageSender {
    type Target = Sender<FromECSMessages>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PhysicsProcessMessageSender {
    pub fn new(physics_process_message_sender: Sender<FromECSMessages>) -> Self {
        PhysicsProcessMessageSender(physics_process_message_sender)
    }
}
