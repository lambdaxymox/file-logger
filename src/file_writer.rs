use chrono::prelude::Utc;
use log_buffer::LogBuffer;
use std::fmt::Write as FmtWrite;
use std::fs::OpenOptions;
use std::io;
use std::io::Write as IoWrite;
use std::path::Path;


#[derive(Debug)]
pub struct FileWriter {
    /// A buffer for storing dates with zero allocation.
    date_buffer: LogBuffer<[u8; 32]>,
    /// A ring buffer for expanding out the current record with zero allocation.
    record_buffer: LogBuffer<Vec<u8>>,
    /// The ring buffer for storing messages.
    buffer: LogBuffer<Vec<u8>>,
}

impl FileWriter {
    pub fn new() -> FileWriter {
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

    pub fn write(&mut self, record: &log::Record, file: &Path) -> io::Result<()> {
        let date = Utc::now();
        self.date_buffer.clear();
        write!(self.date_buffer, "[{}]", date).unwrap();
        self.record_buffer.clear();
        write!(self.record_buffer, "{}", record.args()).unwrap();

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

        Ok(())
    }

    pub fn flush(&mut self, log_file: &Path) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(log_file)?;

        let result = write!(file, "{}", self.buffer.extract());
        self.buffer.clear();

        result
    }
}
