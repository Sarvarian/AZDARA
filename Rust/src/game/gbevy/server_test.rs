use super::Viewport;
use bevy::prelude::*;

pub struct ServerTest;

impl Plugin for ServerTest {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(ready.system());
    }
}

fn ready(viewport: Res<Viewport>, commands: &mut Commands) {
    use gdnative::api::*;

    unsafe {
        let ps = Physics2DServer::godot_singleton();

        let body = ps.body_create();
        ps.body_set_mode(body, Physics2DServer::BODY_MODE_RIGID);

        let shape = RectangleShape2D::new();
        let vec2 = euclid::Vector2D::new(10f32, 10f32);
        shape.set_extents(vec2);

        let trans2 = euclid::Transform2D::<f32, euclid::UnknownUnit, euclid::UnknownUnit>::new(
            1f32, 0f32, 0f32, 1f32, 0f32, 0f32,
        );
        ps.body_add_shape(body, shape.get_rid(), trans2, false);

        let world2d = gdnative::api::World2D::new();
        ps.body_set_space(body, world2d.space());

        let trans2 = euclid::Transform2D::<f32, euclid::UnknownUnit, euclid::UnknownUnit>::new(
            10f32, 0f32, 0f32, 20f32, 0f32, 0f32,
        );
        ps.body_set_state(body, Physics2DServer::BODY_STATE_TRANSFORM, trans2);

        let vs = VisualServer::godot_singleton();

        let c_rid = vs.canvas_create();
        let ci_rid = vs.canvas_item_create();

        vs.viewport_attach_canvas(viewport.clone(), c_rid);
        vs.canvas_item_set_parent(ci_rid, c_rid);

        let rl = gdnative::api::ResourceLoader::godot_singleton();

        let sprite = rl
            .load(
                gdnative::prelude::GodotString::from_str("res://icon.png"),
                "",
                false,
            )
            .unwrap();
        let sprite_rid = sprite.assume_safe().get_rid();

        vs.canvas_item_add_texture_rect(
            ci_rid,
            euclid::Rect::new(
                euclid::Point2D::new(32f32, 32f32),
                euclid::Size2D::new(64f32, 64f32),
            ),
            sprite_rid,
            false,
            gdnative::prelude::Color::rgba(1f32, 1f32, 1f32, 1f32),
            false,
            sprite_rid,
        );

        commands.spawn((
            ResourceHolder,
            SpriteRes {
                gref: sprite,
                rid: sprite_rid,
            },
        ));
    }
}

struct ResourceHolder(u64);

struct SpriteRes {
    gref: gdnative::prelude::Ref<gdnative::api::Resource, gdnative::prelude::Shared>,
    rid: gdnative::prelude::Rid,
}
