mod alert;
pub mod alert_message;
mod file_log;

pub use alert_message::AlertMessage;
use gdnative::{api::DynamicFont, prelude::*};

type Owner = CanvasLayer;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Log {
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
}

#[methods]
impl Log {
    fn new(owner: &Owner) -> Self {
        owner.set_layer(100);
        Log {
            font_bold_italics: None,
            font_italics: None,
            font_bold: None,
            font_normal: None,
            file_log: file_log::FileLog::new(),
            alert: alert::Alert::new(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Owner) {
        self.alert.set_font(self.font_normal.clone());
        owner.add_child(self.alert.get_root(), true);
        self.alert
            .alert("Welcome to AZDARA!", Color::rgb(1f32, 0f32, 0f32));
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
}
