pub struct PhysicsServer(&'static gdnative::api::PhysicsServer);

impl std::ops::Deref for PhysicsServer {
    type Target = gdnative::api::PhysicsServer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PhysicsServer {
    pub fn new() -> PhysicsServer {
        PhysicsServer(unsafe { gdnative::api::PhysicsServer::godot_singleton() })
    }
}
