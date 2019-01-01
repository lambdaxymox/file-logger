use chrono::prelude::Utc;
use std::fmt::Write as FmtWrite;
use std::fs::OpenOptions;
use std::io;
use std::io::Write as IoWrite;
use std::path::Path;


#[derive(Debug)]
pub struct FileWriter {}

impl FileWriter {
    pub fn new() -> FileWriter {
        FileWriter {}
    }

    pub fn write(&self, record: &log::Record, file: &Path) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file)?;

        let date = Utc::now();
        let result = write!(file, "[{}] {}", date, record.args());

        result
    }

    pub fn flush(&self, log_file: &Path) -> io::Result<()> {
        Ok(())
    }
}
