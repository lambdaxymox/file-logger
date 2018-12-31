use chrono::prelude::Utc;
use log;
use log_buffer::LogBuffer;
use std::fmt::Write as FmtWrite;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};


#[derive(Debug)]
struct FileWriter {
    /// A buffer for storing dates with zero allocation.
    date_buffer: LogBuffer<[u8; 32]>,
    /// A ring buffer for expanding out the current record with zero allocation.
    record_buffer: LogBuffer<Vec<u8>>,
    /// The ring buffer for storing messages.
    buffer: LogBuffer<Vec<u8>>,
}

impl FileWriter {
    fn new() -> FileWriter {
        // Create a date buffer.
        let date_storage = [0x00 as u8; 32];
        // Create a record buffer.
        let record_storage = vec![0x00 as u8; 4096];
        // Create an 8kb buffer for storing the finalized messages.
        let storage = vec![0x00 as u8; 8192];

        FileWriter {
            date_buffer: LogBuffer::new(date_storage),
            record_buffer: LogBuffer::new(record_storage),
            buffer: LogBuffer::new(storage),
        }
    }

    fn write(&mut self, record: &log::Record, file: &Path) {
        let date = Utc::now();
        self.date_buffer.clear();
        write!(self.date_buffer, "[{}]", date);
        self.record_buffer.clear();
        write!(self.record_buffer, "{}", record.args());

        let len_date = self.date_buffer.len();
        let len_text = self.record_buffer.len();
        let len_record = len_date + len_text + 2;
        if len_record >= self.buffer.space_remaining() {
            self.flush(file);
        }

        writeln!(
            self.buffer,
            "{} {}", self.date_buffer.extract(), self.record_buffer.extract()
        );
    }

    fn flush(&mut self, log_file: &Path) {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(log_file);

        if file.is_err() {
            eprintln!(
                "ERROR: Could not open the file {} for writing.",
                log_file.display()
            );

            return;
        }

        let mut file = file.unwrap();
        write!(file, "{}", self.buffer.extract()).unwrap();
        self.buffer.clear();
    }
}


#[derive(Debug)]
pub struct FileLogger {
    /// The path to the logging file.
    log_file: PathBuf,
    /// The logging level. This determines what level to filter messages at.
    level: log::Level,
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
