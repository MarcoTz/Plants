use super::init::Logger;
use chrono::Local;
use log::{Level, Log, Metadata, Record};
use std::{
    fs::{remove_file, File, OpenOptions},
    io::Write,
    path::Path,
};

pub struct FileLogger {
    pub level: Level,
    pub file_path: &'static str,
}

impl Logger for FileLogger {
    fn setup(&self) -> Result<(), String> {
        let log_path = Path::new(self.file_path);
        if log_path.exists() {
            remove_file(log_path).map_err(|_| format!("Could not remove {}", self.file_path))?;
        }
        File::create(log_path).map_err(|_| format!("Could not create {}", self.file_path))?;
        Ok(())
    }
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let current_time = Local::now();
            let log_line = format!("{}:{} - {}\n", current_time, record.level(), record.args());
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.file_path)
                .unwrap();
            file.write(log_line.as_bytes()).unwrap();
        }
    }

    fn flush(&self) {}
}
