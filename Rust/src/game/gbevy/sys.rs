use super::com::{Movable, Player};
use bevy::prelude::*;

pub fn setup(commands: &mut Commands) {
    let player = (Player, Movable { x: 5, y: 5 });
    commands.spawn(player);
}
