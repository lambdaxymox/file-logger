use crate::file_writer::FileWriter;
use log;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};


#[derive(Debug)]
pub struct FileLogger {
    /// The path to the logging file.
    log_file: PathBuf,
    /// The logging level. This determines what level to filter messages at.
    level: log::Level,
    /// The file writer. The endpoint where the messages are written.
    writer: Arc<RwLock<FileWriter>>,
}

impl FileLogger {
    ///
    /// Create a new logger.
    ///
    pub fn new<P: AsRef<Path>>(log_file: P, level: log::Level) -> FileLogger {
        FileLogger {
            log_file: log_file.as_ref().to_path_buf(),
            level: level,
            writer: Arc::new(RwLock::new(FileWriter::new())),
        }
    }
}

impl log::Log for FileLogger {
    ///
    /// Determine whether a message would get logged.
    ///
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    ///
    /// Write a message to the log file.
    ///
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let lock = self.writer.as_ref();
            let mut writer = lock.write().unwrap();
            writer.write(record, &self.log_file);
        }
    }

    ///
    /// Finish writing to a log. This function is used to place any final
    /// information in a log file before the logger goes out of scope.
    ///
    fn flush(&self) {
        let lock = self.writer.as_ref();
        let mut writer = lock.write().unwrap();
        writer.flush(&self.log_file);
    }
}

///
/// Initialize a file logger with the specified logging level.
///
pub fn init_with_level<P: AsRef<Path>>(
    log_file: P, level: log::Level) -> Result<(), log::SetLoggerError> {
    
    let logger = FileLogger::new(log_file, level);
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}

///
/// Initialize a file logger that logs all messages by default.
///
pub fn init<P: AsRef<Path>>(log_file: P) -> Result<(), log::SetLoggerError> {
    init_with_level(log_file, log::Level::Trace)
}
