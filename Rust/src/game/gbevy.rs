use bevy::prelude::*;

pub struct GBevy;

impl Plugin for GBevy {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

impl GBevy {
    pub fn register(builder: &gdnative::prelude::ClassBuilder<super::Game>) {
        use euclid::*;
        use gdnative::prelude::*;
        builder.add_signal(Signal {
            name: "spawn_player",
            args: &[SignalArgument {
                name: "position",
                default: Variant::from_vector2(&Vector2D::new(5f32, 5f32)),
                export_info: ExportInfo::new(VariantType::Vector2),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    pub fn ready(world: &World, owner: &gdnative::prelude::Node) {}

    pub fn update(world: &World, owner: &gdnative::prelude::Node) {}
}

fn setup(commands: &mut Commands) {
    let player = (Player, Movable { x: 5, y: 5 });
    commands.spawn(player);
}

struct Player;

struct Movable {
    x: u8,
    y: u8,
}

struct Static {
    x: u8,
    y: u8,
}
