use gdnative::{
    api::{Button, Control, DynamicFont, Panel, RichTextLabel, StyleBoxFlat},
    core_types::{Color, GodotString},
    prelude::Shared,
    Ref,
};

pub struct Console {
    root: Ref<Control, Shared>,
    wall: Ref<RichTextLabel, Shared>,
    close: Ref<Button, Shared>,
}

impl Console {
    pub fn new() -> Self {
        // Prepare root control.
        let root = Control::new();
        root.set_anchor(2, 1.0, false, true);
        root.set_anchor(3, 1.0, false, true);
        root.set_name("Console");

        // Prepare background style box flat
        let style_box_flat = StyleBoxFlat::new();
        style_box_flat.set_bg_color(Color::rgba(0.0, 0.0, 0.0, 0.68));

        // Prepare background
        let background = Panel::new();
        background.set_anchor(2, 1.0, false, true);
        background.set_anchor(3, 1.0, false, true);
        background.add_stylebox_override("panel", style_box_flat);
        background.set_name("Background");

        // Prepare wall
        let wall = RichTextLabel::new();
        wall.set_anchor(0, 0.05, false, true);
        wall.set_anchor(1, 0.03, false, true);
        wall.set_anchor(2, 0.95, false, true);
        wall.set_anchor(3, 0.89, false, true);
        wall.set_selection_enabled(true);
        wall.set_use_bbcode(true);
        wall.set_scroll_follow(true);
        wall.set_name("Wall");

        // Prepare close button
        let close = Button::new();
        //close.set_size(Vector2::new(30.0, 30.0), false);
        close.set_anchor(0, 0.95, false, true);
        close.set_anchor(1, 0.0, false, true);
        close.set_anchor(2, 1.0, false, true);
        close.set_anchor(3, 0.1, false, true);
        close.set_button_mask(7);
        close.set_focus_mode(0);
        close.set_text_align(1);
        close.set_name("Close");
        close.set_text("X");

        // Assemble nodes
        root.add_child(background, true);
        let wall = wall.into_shared();
        root.add_child(&wall, true);
        let close = close.into_shared();
        root.add_child(close, true);

        Console {
            root: root.into_shared(),
            wall,
            close,
        }
    }

    pub unsafe fn set_fonts(
        &mut self,
        font_mono: &Option<Ref<DynamicFont, Shared>>,
        font_bold_italics: &Option<Ref<DynamicFont, Shared>>,
        font_italics: &Option<Ref<DynamicFont, Shared>>,
        font_bold: &Option<Ref<DynamicFont, Shared>>,
        font_normal: &Option<Ref<DynamicFont, Shared>>,
    ) {
        set_font(&mut self.wall, "mono_font", font_mono);
        set_font(&mut self.wall, "bold_italics_font", font_bold_italics);
        set_font(&mut self.wall, "italics_font", font_italics);
        set_font(&mut self.wall, "bold_font", font_bold);
        set_font(&mut self.wall, "normal_font", font_normal);
    }

    pub fn get_root(&self) -> &Ref<Control, Shared> {
        &self.root
    }

    pub fn get_close(&self) -> &Ref<Button, Shared> {
        &self.close
    }

    pub fn get_wall(&self) -> &Ref<RichTextLabel, Shared> {
        &self.wall
    }

    pub unsafe fn open(&mut self) {
        self.root.assume_safe().set_visible(true);
    }

    pub unsafe fn close(&mut self) {
        let root = self.root.assume_safe();
        root.set_visible(false);
        root.release_focus();
    }
}

unsafe fn set_font(
    wall: &mut Ref<RichTextLabel, Shared>,
    name: impl Into<GodotString>,
    font: &Option<Ref<DynamicFont, Shared>>,
) {
    if let Some(font) = font {
        wall.assume_safe().add_font_override(name, font);
    }
}
