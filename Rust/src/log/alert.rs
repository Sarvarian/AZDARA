use gdnative::{
    api::{Control, DynamicFont, Label, VBoxContainer},
    core_types::{Color, GodotString},
    prelude::{NativeClass, Shared, Unique},
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
        // Create root node.
        let root = Control::new();
        root.set_anchor(0, 0.03, false, true);
        root.set_anchor(1, 0.05, false, true);
        root.set_anchor(2, 0.97, false, true);
        root.set_anchor(3, 0.95, false, true);
        root.set_h_grow_direction(0);
        root.set_v_grow_direction(0);
        root.set_mouse_filter(2);
        root.set_name("Alert");

        // Create vertical box container.
        let vbox = VBoxContainer::new();
        vbox.set_anchor(2, 1.0, false, true);
        vbox.set_anchor(3, 1.0, false, true);
        vbox.set_h_grow_direction(0);
        vbox.set_v_grow_direction(0);
        vbox.set_mouse_filter(2);
        vbox.set_alignment(2);
        vbox.set_name("VBox");

        // Add vbox as child of root
        let vbox = vbox.into_shared();
        root.add_child(vbox, false);

        // Create final object.
        Alert {
            font: None,
            root: root.into_shared(),
            vbox,
        }
    }

    pub fn set_font(&mut self, font: Option<Ref<DynamicFont, Shared>>) {
        self.font = font;
    }

    pub fn get_root(&self) -> &Ref<Control, Shared> {
        &self.root
    }

    pub fn get_vbox(&self) -> &Ref<VBoxContainer, Shared> {
        &self.vbox
    }

    pub fn create_alert_message(
        &mut self,
        msg: impl Into<GodotString>,
        color: Color,
    ) -> Ref<Label, Unique> {
        // Create instance.
        let label = AlertMessage::new_instance();
        // Get base label.
        let label = label.into_base();
        // Setup label
        label.set_text(msg);
        label.set_modulate(color);
        label.set_autowrap(true);
        // Set font if we have one.
        if let Some(font) = &self.font {
            label.add_font_override("font", font);
        }
        // Return node
        label
    }
}
