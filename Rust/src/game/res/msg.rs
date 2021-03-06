use super::super::msg::*;
use std::sync::mpsc::Sender;

pub struct ProcessMessageSender(Sender<ECSMessages>);

impl std::ops::Deref for ProcessMessageSender {
    type Target = Sender<ECSMessages>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProcessMessageSender {
    pub fn new(process_message_sender: Sender<ECSMessages>) -> Self {
        ProcessMessageSender(process_message_sender)
    }
}

pub struct PhysicsProcessMessageSender(Sender<ECSMessages>);

impl std::ops::Deref for PhysicsProcessMessageSender {
    type Target = Sender<ECSMessages>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PhysicsProcessMessageSender {
    pub fn new(physics_process_message_sender: Sender<ECSMessages>) -> Self {
        PhysicsProcessMessageSender(physics_process_message_sender)
    }
}
