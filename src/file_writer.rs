use chrono::prelude::Utc;
use std::fs::OpenOptions;
use std::io;
use std::io::Write as IoWrite;
use std::path::PathBuf;


#[derive(Debug)]
pub struct FileWriter {
    /// The path to the logging file.
    file: PathBuf,
}

impl FileWriter {
    pub fn new(file: PathBuf) -> FileWriter {
        FileWriter {
            file: file,
        }
    }

    pub fn write(&self, record: &log::Record) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.file)?;

        let date = Utc::now();
        let result = write!(file, "[{}] {}", date, record.args());

        result
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}
