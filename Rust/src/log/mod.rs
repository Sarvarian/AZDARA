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
    unsafe fn _ready(&mut self, owner: &Owner) {
        // Set fonts.
        self.refresh_fonts(owner);

        // Prepare alert.
        owner.add_child(self.alert.get_root(), true);

        // Prepare console.
        owner.add_child(self.console.get_root(), true);
        connect_close_button_to_owner(self, owner);
        self.console.close();
    }

    #[export]
    unsafe fn _input(&mut self, _: &Owner, event: Ref<InputEvent, Shared>) {
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

    #[export]
    unsafe fn _on_close_pressed(&mut self, _: &Owner) {
        self.console.close();
    }

    #[export]
    fn check(&self, _: &Owner) -> bool {
        true
    }

    #[export]
    unsafe fn refresh_fonts(&mut self, _: &Owner) {
        self.alert.set_font(self.font_normal.clone());
        self.console.set_fonts(
            &self.font_mono,
            &self.font_bold_italics,
            &self.font_italics,
            &self.font_bold,
            &self.font_normal,
        );
    }

    #[export]
    unsafe fn alert(&mut self, _: &Owner, msg: GodotString, color: Color) {
        let node = self.alert.create_alert_message(msg, color);
        self.alert.get_vbox().assume_safe().add_child(node, false);
    }

    #[export]
    fn error(&mut self, owner: &Owner, msg: GodotString) {
        // Get time.
        let datetime = chrono::Local::now().to_string();
        let full_message = format!("{} {}", &datetime, &msg);

        // Log to file.
        if let Ok(file_log) = &mut self.file_log {
            file_log.log_without_time(full_message);
        }

        // Log to alert and console.
        unsafe {
            self.log_put(owner, datetime.into(), msg, Color::rgb(1.0, 0.0, 0.0));
        }
    }

    #[export]
    pub unsafe fn put(&mut self, _: &Owner, msg: GodotString, color: Color) {
        let wall = self.console.get_wall().assume_safe();
        let _ = wall.append_bbcode("\n");
        wall.push_color(color);
        let _ = wall.append_bbcode(msg);
        wall.pop();
    }

    #[export]
    pub unsafe fn log_put(
        &mut self,
        owner: &Owner,
        datetime: GodotString,
        msg: GodotString,
        color: Color,
    ) {
        // Alert.
        self.alert(owner, msg.clone(), color);

        // Put on console.
        let wall = self.console.get_wall().assume_safe();
        let _ = wall.append_bbcode("\n");
        wall.push_color(Color::rgb(0.0, 1.0, 0.0));
        let _ = wall.append_bbcode(datetime);
        let _ = wall.pop();
        let _ = wall.append_bbcode(" ");
        wall.push_color(color);
        let _ = wall.append_bbcode(msg);
        wall.pop();
    }
}

unsafe fn connect_close_button_to_owner(log: &mut Log, owner: &Owner) {
    // Name of group.
    const GROUP_NAME: &str = "ConnectConsoleCloseButtonToThisObject";

    // Get close button
    let close = log.console.get_close();

    // Add owner to group.
    owner.add_to_group(GROUP_NAME, false);

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

    // Remove owner from group.
    owner.remove_from_group(GROUP_NAME)
}
