use gdnative::{api::DynamicFont, prelude::Shared, Ref};

pub struct Alert {
    font: Option<Ref<DynamicFont, Shared>>,
}

impl Alert {
    pub fn new(font: Option<Ref<DynamicFont, Shared>>) -> Self {
        Alert { font }
    }

    pub fn set_font(&mut self, font: Option<Ref<DynamicFont, Shared>>) {
        self.font = font;
    }
}
