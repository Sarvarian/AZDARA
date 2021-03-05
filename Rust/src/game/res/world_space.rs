use gdnative::core_types::Rid;

pub struct WorldSpace(Rid);

impl std::ops::Deref for WorldSpace {
    type Target = Rid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WorldSpace {
    pub fn new(space: Rid) -> WorldSpace {
        WorldSpace(space)
    }
}
