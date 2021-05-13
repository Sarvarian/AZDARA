pub type SpeedType = i16;

pub struct MovementSpeed(SpeedType);

impl std::ops::Deref for MovementSpeed {
    type Target = SpeedType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MovementSpeed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MovementSpeed {
    pub fn new(speed: SpeedType) -> Self {
        MovementSpeed(speed)
    }
}
