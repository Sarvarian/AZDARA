pub struct ResourceLoader(&'static gdnative::api::ResourceLoader);

impl std::ops::Deref for ResourceLoader {
    type Target = &'static gdnative::api::ResourceLoader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ResourceLoader {
    pub fn new() -> Self {
        ResourceLoader(gdnative::api::ResourceLoader::godot_singleton())
    }
}
