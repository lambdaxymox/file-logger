use chrono::prelude::Utc;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::PathBuf;


#[derive(Debug)]
pub struct FileWriter {
    /// The path to the logging file.
    path: PathBuf,
    writer: BufWriter<File>,
}

impl FileWriter {
    pub fn new(path: PathBuf) -> FileWriter {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path.as_path()).unwrap();

        FileWriter {
            path: path,
            writer: BufWriter::new(file),
        }
    }

    pub fn write(&mut self, record: &log::Record) -> io::Result<()> {
        let writer = self.writer.get_mut();
        let date = Utc::now();
        let result = writeln!(writer, "[{}] {}", date, record.args());

        result
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}
