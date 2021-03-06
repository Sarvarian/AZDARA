pub struct GodotServerResourceManager {
    _physics_server: &'static gdnative::api::PhysicsServer,
    _visual_server: &'static gdnative::api::VisualServer,
}

impl GodotServerResourceManager {
    pub fn new() -> Self {
        GodotServerResourceManager {
            _physics_server: unsafe { gdnative::api::PhysicsServer::godot_singleton() },
            _visual_server: unsafe { gdnative::api::VisualServer::godot_singleton() },
        }
    }
}
