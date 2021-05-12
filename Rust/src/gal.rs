use gdnative::{
    api::{Resource, VisualServer},
    prelude::*,
};

pub struct Sprite(Rid, Ref<Resource, Shared>);

pub fn make_sprite(texture_path: GodotString, canvas: Rid) -> Result<Sprite, GodotError> {
    let rl = gdnative::api::ResourceLoader::godot_singleton();
    let texture = rl.load(texture_path, "", false).ok_or(GodotError::Failed)?;

    let rect = {
        let size = unsafe { texture.assume_safe().call("get_size", &[]) };
        if let Some(size) = size.try_to_vector2() {
            euclid::rect(0f32, 0f32, size.x, size.y)
        } else {
            godot_error!("On make_sprite Getting Size Failed!");
            euclid::rect(1f32, 1f32, 1f32, 1f32)
        }
    };

    let vs = unsafe { VisualServer::godot_singleton() };

    let item = vs.canvas_item_create();
    vs.canvas_item_set_parent(item, canvas);

    let texture_rid = unsafe { texture.assume_safe().get_rid() };

    vs.canvas_item_add_texture_rect(
        item,
        rect,
        texture_rid,
        false,
        Color::rgba(1f32, 1f32, 1f32, 1f32),
        false,
        texture_rid,
    );

    Ok(Sprite(item, texture))
}
