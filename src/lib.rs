//! The file logger crate is a library for a logger that prints all
//! messages with a readable output format to a file.

extern crate chrono;
extern crate log;
extern crate log_buffer;

mod file_logger;
mod file_writer;

pub use self::file_logger::*;
