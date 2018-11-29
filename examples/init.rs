extern crate log;
extern crate file_logger;

use log::{trace};
use std::fs;
use std::path::Path;


fn main() {
    let log_file = Path::new("init.log");
    file_logger::init(log_file).unwrap();
    trace!("This is an example message.");
    assert!(log_file.exists());
    fs::remove_file(log_file).unwrap();
}
