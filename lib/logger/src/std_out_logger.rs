use super::init::Logger;
use chrono::Local;
use log::{Level, Log, Metadata, Record};

pub struct StdOutLogger {
    pub level: Level,
}

impl Logger for StdOutLogger {
    fn setup(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Log for StdOutLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let current_time = Local::now();
            println!("{}:{} - {}", current_time, record.level(), record.args())
        }
    }

    fn flush(&self) {}
}
