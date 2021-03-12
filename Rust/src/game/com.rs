use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player(u8);

impl std::ops::Deref for Player {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Position(gdnative::core_types::Vector3);

impl std::ops::Deref for Position {
    type Target = gdnative::core_types::Vector3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
