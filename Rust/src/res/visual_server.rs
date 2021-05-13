pub struct VisualServer();

impl std::ops::Deref for VisualServer {
    type Target = gdnative::api::VisualServer;

    fn deref(&self) -> &Self::Target {
        unsafe { gdnative::api::VisualServer::godot_singleton() }
    }
}

impl std::default::Default for VisualServer {
    fn default() -> Self {
        VisualServer()
    }
}
