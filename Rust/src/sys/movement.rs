use com::Transform2D;
use gdnative::godot_print;
use legion::{systems::CommandBuffer, *};

use crate::{com, res};

pub trait MovementSystems {
    fn add_movement_systems(&mut self) -> &mut Self;
}

impl MovementSystems for super::Builder3 {
    fn add_movement_systems(&mut self) -> &mut Self {
        self.physics_process
            .add_system(update_item_transform_system())
            .add_system(resolve_input_move_dir_system());
        self
    }
}

#[system(for_each)]
fn resolve_input_move_dir(
    #[resource] delta: &res::Delta,
    cmd: &mut CommandBuffer,
    entity: &Entity,
    transform: &mut Transform2D,
    dir: &com::InputMoveDir,
    speed: &com::MovementSpeed,
) {
    transform.m31 += dir.x * (**delta as f32) * (**speed as f32);
    transform.m32 += dir.y * (**delta as f32) * (**speed as f32);
    cmd.remove_component::<com::InputMoveDir>(entity.clone());
}

#[system(for_each)]
#[filter(maybe_changed::<Transform2D>())]
fn update_item_transform(
    #[resource] vs: &res::VisualServer,
    item: &com::CanvasItem,
    transform: &Transform2D,
) {
    item.set_transform(transform.clone(), vs);
    godot_print!("{} {}", transform.m31, transform.m32);
}
