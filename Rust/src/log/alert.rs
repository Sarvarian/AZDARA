use gdnative::{
    api::{Control, DynamicFont, VBoxContainer},
    core_types::{Color, GodotString},
    prelude::{NativeClass, Shared},
    Ref,
};

use super::AlertMessage;

pub struct Alert {
    font: Option<Ref<DynamicFont, Shared>>,
    root: Ref<Control, Shared>,
    vbox: Ref<VBoxContainer, Shared>,
}

impl Alert {
    pub fn new() -> Self {
        let root = Control::new();
        root.set_anchor(0, 0.03, false, true);
        root.set_anchor(1, 0.05, false, true);
        root.set_anchor(2, 0.97, false, true);
        root.set_anchor(3, 0.95, false, true);
        root.set_h_grow_direction(0);
        root.set_v_grow_direction(0);
        root.set_mouse_filter(2);
        root.set_name("Alert");

        let vbox = VBoxContainer::new();
        vbox.set_anchor(2, 1.0, false, true);
        vbox.set_anchor(3, 1.0, false, true);
        vbox.set_h_grow_direction(0);
        vbox.set_v_grow_direction(0);
        vbox.set_mouse_filter(2);
        vbox.set_alignment(2);
        vbox.set_name("VBox");

        let vbox = vbox.into_shared();
        root.add_child(vbox, false);

        Alert {
            font: None,
            root: root.into_shared(),
            vbox: vbox,
        }
    }

    pub fn set_font(&mut self, font: Option<Ref<DynamicFont, Shared>>) {
        self.font = font;
    }

    pub fn get_root(&self) -> &Ref<Control, Shared> {
        &self.root
    }

    pub fn alert(&mut self, msg: impl Into<GodotString>, color: Color) {
        let label = AlertMessage::new_instance();
        let label = label.into_base();
        label.set_text(msg);
        label.set_modulate(color);
        if let Some(font) = &self.font {
            label.add_font_override("font", font);
        }

        unsafe { self.vbox.assume_safe().add_child(label, false) }
    }
}