use gdnative::{
    api::{Image, VisualServer},
    core_types::{Color, GodotError, GodotString, Rid},
};

pub struct GodotServerResourceManager {
    // visual_server: &'static VisualServer,
    canvas: Rid,
}

impl GodotServerResourceManager {
    pub fn new(canvas: Rid) -> Self {
        GodotServerResourceManager {
            // visual_server: unsafe { VisualServer::godot_singleton() },
            canvas,
        }
    }

    pub fn make_sprite(&self, image_path: GodotString) -> Result<Rid, GodotError> {
        let image = Image::new();
        image.load(image_path)?;

        let rect = euclid::rect(0f32, 0f32, image.get_size().x, image.get_size().y);

        let vs = unsafe { VisualServer::godot_singleton() };

        let texture = vs.texture_create();
        vs.texture_set_data(texture, image, 0);

        let item = vs.canvas_item_create();

        vs.canvas_item_add_texture_rect(
            item,
            rect,
            texture,
            false,
            Color::rgba(1f32, 1f32, 1f32, 1f32),
            false,
            texture,
        );

        vs.canvas_item_set_parent(item, self.canvas);

        Ok(item)
    }
}
