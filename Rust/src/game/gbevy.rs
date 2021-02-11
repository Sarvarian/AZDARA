use bevy::prelude::*;

pub struct GBevy;

impl Plugin for GBevy {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(commands: &mut Commands) {
    let player = (Player, Position { x: 5, y: 5 });
    commands.spawn(player);
}

struct Player;

struct Position {
    x: u8,
    y: u8,
}
