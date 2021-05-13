pub struct Input(&'static gdnative::api::Input);

impl std::ops::Deref for Input {
    type Target = &'static gdnative::api::Input;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::default::Default for Input {
    fn default() -> Self {
        Input(gdnative::api::Input::godot_singleton())
    }
}
