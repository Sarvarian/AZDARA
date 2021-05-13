pub struct ResourceLoader(&'static gdnative::api::ResourceLoader);

impl std::ops::Deref for ResourceLoader {
    type Target = &'static gdnative::api::ResourceLoader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::default::Default for ResourceLoader {
    fn default() -> Self {
        ResourceLoader(gdnative::api::ResourceLoader::godot_singleton())
    }
}
