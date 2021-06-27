mod alert;
pub mod alert_message;
mod console;
mod file_log;

pub use alert_message::AlertMessage;
use gdnative::{api::DynamicFont, prelude::*};

type Owner = CanvasLayer;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Log {
    #[property]
    font_mono: Option<Ref<DynamicFont, Shared>>,

    #[property]
    font_bold_italics: Option<Ref<DynamicFont, Shared>>,

    #[property]
    font_italics: Option<Ref<DynamicFont, Shared>>,

    #[property]
    font_bold: Option<Ref<DynamicFont, Shared>>,

    #[property]
    font_normal: Option<Ref<DynamicFont, Shared>>,

    file_log: Result<file_log::FileLog, GodotError>,
    alert: alert::Alert,
    console: console::Console,
}

#[methods]
impl Log {
    fn new(owner: &Owner) -> Self {
        owner.set_layer(100);
        Log {
            font_mono: None,
            font_bold_italics: None,
            font_italics: None,
            font_bold: None,
            font_normal: None,
            file_log: file_log::FileLog::new(),
            alert: alert::Alert::new(),
            console: console::Console::new(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Owner) {
        // Set fonts
        self.refresh_fonts(owner);

        // Prepare alert.
        owner.add_child(self.alert.get_root(), true);
        self.alert
            .alert("Welcome to AZDARA!", Color::rgb(1.0, 0.1, 0.6));

        // Prepare console.
        owner.add_child(self.console.get_root(), true);
        connect_close_button_to_owner(self, owner);
        self.console.close();
    }

    #[export]
    fn _input(&mut self, _owner: &Owner, event: Ref<InputEvent, Shared>) {
        // Enter unsafe.
        unsafe {
            // Assume event as safe.
            let event = event.assume_safe();

            // Check for visibility of console.
            if self.console.get_root().assume_safe().is_visible() {
                // If visible and action released, open the console.
                if event.is_action_released("console") {
                    self.console.close();
                }
            } else {
                // If not visible and action released, close the console.
                if event.is_action_released("console") {
                    self.console.open();
                }
            }
        }
        // Exit unsafe.
    }

    #[export]
    fn _on_close_pressed(&mut self, _owner: &Owner) {
        self.console.close();
    }

    #[export]
    fn check(&self, _owner: &Owner) -> bool {
        true
    }

    #[export]
    fn error(&mut self, _owner: &Owner, msg: GodotString) {
        if let Ok(file_log) = &mut self.file_log {
            file_log.log_without_time(msg.to_string());
        }
    }

    #[export]
    fn refresh_fonts(&mut self, _owner: &Owner) {
        self.alert.set_font(self.font_normal.clone());
        self.console.set_fonts(
            &self.font_mono,
            &self.font_bold_italics,
            &self.font_italics,
            &self.font_bold,
            &self.font_normal,
        );
    }
}

fn connect_close_button_to_owner(log: &mut Log, owner: &Owner) {
    // Name of group.
    const GROUP_NAME: &str = "ConnectConsoleCloseButtonToThisObject";

    // Get close button
    let close = log.console.get_close();

    // Add owner to group.
    owner.add_to_group(GROUP_NAME, false);

    // Entre unsafe.
    unsafe {
        // Extract nodes from group.
        let nodes_in_group = owner
            .get_tree()
            .unwrap()
            .assume_safe()
            .get_nodes_in_group(GROUP_NAME);

        // Iterate trough nodes.
        for node in nodes_in_group.into_iter() {
            // Cast node to object.
            let object: Ref<Object, Shared> = node.try_to_object().unwrap();

            // Connect close button "pressed" signal to object.
            close
                .assume_safe()
                .connect(
                    "pressed",
                    object,
                    "_on_close_pressed",
                    VariantArray::new_shared(),
                    1,
                )
                .unwrap();
        } // End of iteration.
    }
    // Exit unsafe.

    // Remove owner from group.
    owner.remove_from_group(GROUP_NAME)
}
