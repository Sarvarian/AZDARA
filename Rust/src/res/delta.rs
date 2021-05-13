pub struct Delta(f64);

impl std::ops::Deref for Delta {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Delta {
    fn default() -> Self {
        Delta(0f64)
    }
}

impl Delta {
    pub fn set(&mut self, value: f64) {
        self.0 = value
    }
}
