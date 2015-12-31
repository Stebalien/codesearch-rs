/// custom logger

use chrono::Local;
use log::{self, Log, LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            let now_time = now.format("%Y/%m/%d %H:%M:%S");
            println!("{} {}", now_time, record.args());
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(Logger)
    })
}
