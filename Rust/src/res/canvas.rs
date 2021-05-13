use gdnative::core_types::Rid;

pub struct Canvas(Rid);

impl std::ops::Deref for Canvas {
    type Target = Rid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Canvas {
    pub fn new(canvas: Rid) -> Self {
        Canvas(canvas)
    }
}
