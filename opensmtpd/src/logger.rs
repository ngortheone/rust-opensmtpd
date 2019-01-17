use crate::errors::Error;
use log::{Level, Metadata, Record};
use std::io::{self, Write};

#[derive(Clone)]
pub struct SmtpdLogger {
    level: Level,
}

impl SmtpdLogger {
    pub fn new() -> Self {
        SmtpdLogger::default()
    }

    pub fn set_level(&mut self, level: Level) -> Self {
        self.level = level;
        self.clone()
    }

    pub fn init(self) -> Result<(), Error> {
        let level = self.level.to_level_filter();
        log::set_boxed_logger(Box::new(self))?;
        log::set_max_level(level);
        Ok(())
    }
}

impl Default for SmtpdLogger {
    fn default() -> Self {
        SmtpdLogger { level: Level::Warn }
    }
}

impl log::Log for SmtpdLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let stderr = io::stderr();
            let mut handle = stderr.lock();
            writeln!(handle, "{}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}
