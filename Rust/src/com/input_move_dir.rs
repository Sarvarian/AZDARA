use gdnative::core_types::Vector2;

pub struct InputMoveDir(pub Vector2);

impl std::ops::Deref for InputMoveDir {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for InputMoveDir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
