use gdnative::core_types::Vector2;

pub struct InputMoveDir(Vector2);

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

impl std::convert::From<Vector2> for InputMoveDir {
    fn from(other: Vector2) -> Self {
        InputMoveDir(other)
    }
}

impl std::convert::From<InputMoveDir> for Vector2 {
    fn from(other: InputMoveDir) -> Self {
        other.0
    }
}
