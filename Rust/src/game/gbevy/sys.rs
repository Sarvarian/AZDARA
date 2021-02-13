use super::com::{District, Movable, Player};
use bevy::prelude::*;

pub fn setup(commands: &mut Commands) {
    let player = (Player, Movable { x: 5, y: 5 });
    commands.spawn(()).with_children(|parent| {
        parent.spawn(player);
    });
}
