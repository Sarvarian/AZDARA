use crate::{com, res};
use gdnative::{core_types::GodotString, godot_error};
use legion::{systems::CommandBuffer, *};

#[system]
pub fn make_sprite(
    #[resource] vs: &res::VisualServer,
    #[resource] rl: &res::ResourceLoader,
    #[resource] canvas: &res::Canvas,
    cmd: &mut CommandBuffer,
) {
    match com::Texture::new(GodotString::from_str("res://contents/icon.tres"), rl) {
        Some(texture) => {
            let canvas = com::CanvasItem::new(**canvas, vs);
            canvas.attach_texture(&texture, vs);
            let transform2d = com::Transform2D::identity();
            let player = com::Player::new(0);
            let speed = com::MovementSpeed::new(500);
            cmd.push((player, speed, canvas, texture, transform2d));
        }
        None => {
            godot_error!("Error make_sprite: Couldn't make new Texture!");
        }
    }
}
