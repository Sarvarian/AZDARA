use gdnative::{api::File, core_types::GodotError, prelude::Unique, Ref};

const LOG_PATH: &str = "user://azdara.log";

pub struct FileLog {
    file: Ref<File, Unique>,
}

impl FileLog {
    pub fn new() -> Result<Self, GodotError> {
        let file = File::new();
        file.open(LOG_PATH, 2)?;
        let mut file_log = FileLog { file };
        file_log.log_with_time("AZDARA LOG START".to_string());
        Ok(file_log)
    }

    pub fn log_with_time(&mut self, msg: String) {
        self.file.seek_end(0);
        let msg = format!("{} {}\n", chrono::Local::now().to_string(), msg);
        self.file.store_string(msg);
    }

    pub fn log_without_time(&mut self, msg: String) {
        self.file.seek_end(0);
        let msg = format!("{}\n", msg);
        self.file.store_string(msg);
    }
}

impl Drop for FileLog {
    fn drop(&mut self) {
        self.log_with_time("AZDARA LOG END".to_string());
        self.file.close();
    }
}
