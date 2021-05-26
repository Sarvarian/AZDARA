use gdnative::{api::File, prelude::*};

type Owner = Node;

const LOG_PATH: &str = "user://azdara.log";

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct Log {
    log_file: Ref<File, Unique>,
}

#[methods]
impl Log {
    fn new(_owner: &Owner) -> Self {
        let log_file = File::new();
        if let Err(err) = log_file.open(LOG_PATH, 2) {
            godot_error!(
                "Open File for Log at '{}' Failed with GodotError '{}'!!!",
                LOG_PATH,
                err
            )
        };
        Log { log_file }
    }

    #[export]
    fn _enter_tree(&mut self, owner: &Owner) {
        self.log_with_time(owner, GodotString::from("AZDARA LOG START"));
    }

    #[export]
    fn _exit_tree(&mut self, owner: &Owner) {
        self.log_with_time(owner, GodotString::from("AZDARA LOG END"));
        self.log_file.close();
    }

    #[export]
    fn log_with_time(&mut self, owner: &Owner, text: GodotString) {
        self.log_file.seek_end(0);
        let text = format!("{} {}\n", self.get_current_datetime(owner), text);
        self.log_file.store_string(text);
    }

    #[export]
    fn log_without_time(&mut self, _owner: &Owner, text: GodotString) {
        self.log_file.seek_end(0);
        let text = format!("{}\n", text);
        self.log_file.store_string(text);
    }

    #[export]
    fn get_current_datetime(&self, _owner: &Owner) -> GodotString {
        let datetime = chrono::Local::now().to_string();
        GodotString::from(datetime)
    }
}