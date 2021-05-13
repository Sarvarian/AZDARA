pub struct Player(u8);

impl std::ops::Deref for Player {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Player {
    pub fn new(index: u8) -> Self {
        Player(index)
    }
}
