use super::{G2BMessage, Receiver};
use bevy::prelude::*;
use std::sync::{self, mpsc};

pub struct ReceiveHandler;

impl Plugin for ReceiveHandler {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(receive_handler.system());
    }
}

fn receive_handler(
    receiver: Res<Receiver>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    loop {
        let msg = match receiver.try_lock() {
            Ok(receiver) => match receiver.try_recv() {
                Ok(msg) => Some(msg),
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => None,
                    mpsc::TryRecvError::Disconnected => None,
                },
            },
            Err(err) => match err {
                sync::TryLockError::WouldBlock => None,
                sync::TryLockError::Poisoned(_) => None,
            },
        };
        if let Some(msg) = msg {
            match msg {
                G2BMessage::Quit => {
                    app_exit_events.send(bevy::app::AppExit);
                }
            }
        } else {
            break;
        }
    }
}
