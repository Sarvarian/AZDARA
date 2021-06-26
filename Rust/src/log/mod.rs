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

    file_log: Option<file_log::FileLog>,
    alert: alert::Alert,
}

#[methods]
impl Log {
    fn new(owner: &Owner) -> Self {
        owner.add_child(create_alert(), true);
        owner.add_child(create_console(), true);
        Log {
            font_bold_italics: None,
            font_italics: None,
            font_bold: None,
            font_normal: None,
            file_log: file_log::FileLog::new().ok(),
            alert: alert::Alert::new(None),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Owner) {
        self.alert.set_font(self.font_normal.clone());

        let am = AlertMessage::new_instance();
        let am = am.into_base();
        am.set_text("Welcome to AZDARA!");
        if let Some(font) = &self.font_normal {
            am.add_font_override("font", font);
        }
        owner.add_child(am, false);
    }

    #[export]
    fn check(&self, _owner: &Owner) -> bool {
        true
    }

    #[export]
    fn error(&mut self, _owner: &Owner, msg: GodotString) {
        if let Some(file_log) = &mut self.file_log {
            file_log.log_without_time(msg.to_string());
        }
    }
}

fn create_alert() -> Ref<Node, Unique> {
    let root = Node::new();

    return root;
}

fn create_console() -> Ref<Node, Unique> {
    let root = Node::new();

    return root;
}
