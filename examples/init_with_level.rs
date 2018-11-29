extern crate log;
extern crate file_logger;

use log::{warn, info};
use log::Level;
use std::fs;
use std::path::Path;


fn main() {
    let log_file = Path::new("init.log");
    file_logger::init_with_level(log_file, Level::Warn).unwrap();
    warn!("This is an example message.");
    info!("This is an example message that does not get written to the log.");
    assert!(log_file.exists());
    fs::remove_file(log_file).unwrap();
}
