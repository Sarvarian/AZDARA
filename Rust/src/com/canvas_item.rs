use crate::com;
use gdnative::{
    api::VisualServer,
    core_types::{Color, Rect2, Rid, Size2, Transform2D},
    godot_error,
};

pub struct CanvasItem(Rid);

impl std::ops::Deref for CanvasItem {
    type Target = Rid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CanvasItem {
    pub fn new(parent_canvas: Rid, visual_server: &VisualServer) -> Self {
        let rid = visual_server.canvas_item_create();
        visual_server.canvas_item_set_parent(rid, parent_canvas);
        CanvasItem(rid)
    }

    pub fn set_transform(&self, transform: Transform2D, visual_server: &VisualServer) {
        visual_server.canvas_item_set_transform(**self, transform);
    }

    pub fn attach_texture(&self, texture: &com::Texture, visual_server: &VisualServer) {
        let rect = {
            let size = unsafe { texture.texture.assume_safe().call("get_size", &[]) };
            if let Some(size) = size.try_to_vector2() {
                Rect2::from_size(size.into())
            } else {
                godot_error!("On make_sprite Getting Size Failed!");
                Rect2::from_size(Size2::new(8f32, 8f32))
            }
        };

        visual_server.canvas_item_add_texture_rect(
            **self,
            rect,
            texture.rid,
            false,
            Color::rgba(1f32, 1f32, 1f32, 1f32),
            false,
            texture.rid,
        );
    }
}
