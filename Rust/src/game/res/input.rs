pub struct Input(&'static gdnative::api::Input);

impl std::ops::Deref for Input {
    type Target = &'static gdnative::api::Input;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Input {
    pub fn new() -> Self {
        Input(gdnative::api::Input::godot_singleton())
    }
}
