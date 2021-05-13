use gdnative::{
    api::{Resource, ResourceLoader},
    core_types::{GodotString, Rid},
    prelude::Shared,
    Ref,
};

pub struct Texture {
    pub rid: Rid,
    pub texture: Ref<Resource, Shared>,
}

impl Texture {
    pub fn new(texture_path: GodotString, resource_loader: &ResourceLoader) -> Option<Self> {
        let texture = resource_loader.load(texture_path, "", false)?;

        let rid = unsafe { texture.assume_safe().get_rid() };

        Some(Texture { rid, texture })
    }
}
