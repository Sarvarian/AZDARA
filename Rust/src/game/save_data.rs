use gdnative::core_types::Dictionary;

pub struct SaveData {
    pub maps: Dictionary,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            maps: Dictionary::default(),
        }
    }
}
