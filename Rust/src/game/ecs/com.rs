pub struct DistrictId(pub u8);

impl std::ops::Deref for DistrictId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Player(pub u8);

impl std::ops::Deref for Player {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Movable {
    pub x: u16,
    pub y: u16,
}

pub struct Static {
    pub x: u16,
    pub y: u16,
}
