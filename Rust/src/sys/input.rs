use gdnative::core_types::Vector2;
use legion::{systems::CommandBuffer, *};

use crate::{com, res};

pub trait InputSystems {
    fn add_input_systems(&mut self) -> &mut Self;
}

impl InputSystems for systems::Builder {
    fn add_input_systems(&mut self) -> &mut Self {
        self.add_system(player_movement_system())
    }
}

#[system(for_each)]
fn player_movement(
    #[resource] input: &res::Input,
    cmd: &mut CommandBuffer,
    entity: &Entity,
    player: &com::Player,
) {
    let right = input.get_action_strength(format!("player{}_move_right", **player));
    let left = input.get_action_strength(format!("player{}_move_left", **player));
    let up = input.get_action_strength(format!("player{}_move_up", **player));
    let down = input.get_action_strength(format!("player{}_move_down", **player));
    let dir = Vector2::new((right - left) as f32, (down - up) as f32).clamp_length(0f32, 1f32);
    if dir.x.abs() > f32::EPSILON || dir.y.abs() > f32::EPSILON {
        cmd.add_component(entity.clone(), com::InputMoveDir(dir));
    }
}
