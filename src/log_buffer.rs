use std::fmt;


#[derive(Debug)]
pub struct LogBuffer<Storage: AsRef<[u8]> + AsMut<[u8]>> {
    buffer: Storage,
    wrapped: bool,
    start: usize,
    end: usize,
}

impl<Storage: AsRef<[u8]> + AsMut<[u8]>> LogBuffer<Storage> {
    pub fn new(storage: Storage) -> LogBuffer<Storage> {
        let mut log_buffer = LogBuffer {
            buffer: storage,
            wrapped: false,
            start: 0,
            end: 0,
        };

        log_buffer.clear();
        log_buffer
    }

    pub fn clear(&mut self) {
        self.wrapped = false;
        self.start = 0;
        self.end = 0;
        for byte in self.buffer.as_mut().iter_mut() {
            *byte = 0x00;
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.start == self.end) && !self.wrapped
    }

    fn rotate(&mut self) {
        if self.wrapped && (self.end <= self.start) {

        } else if self.wrapped && (self.end > self.start) {

        } else {

        }
    }

    pub fn extract(&mut self) -> &str {
        fn is_utf8_leader(byte: u8) -> bool {
            byte & 0b10000000 == 0b00000000 || byte & 0b11100000 == 0b11000000 ||
                byte & 0b11110000 == 0b11100000 || byte & 0b11111000 == 0b11110000
        }

        if self.wrapped {
            self.rotate();
        }

        let buffer = self.buffer.as_mut();
        for i in 0..buffer.len() {
            if is_utf8_leader(buffer[i]) {
                return str::from_utf8(&buffer[i..]).unwrap();
            }
        }

        ""
    }
}
