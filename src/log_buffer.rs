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
        if self.wrapped && (self.start == self.end) {
            self.buffer.rotate_left(self.end);
            self.wrapped = false;
            self.start = 0;
            self.end = self.buffer.len() - 1;
        } else if self.start < self.end {
            self.buffer.rotate_left(self.start);
            self.wrapped = false;
            self.end -= start;
            self.start = 0;
        } else if self.start > self.end {
            self.buffer.rotate_left(self.end);
            self.wrapped = false;
            self.start -= self.end;
            self.end = self.buffer.len() - 1;
            self.buffer.rotate_left(self.start);
            self.end -= self.start;
            self.start = 0;
        } else {
            self.buffer.rotate_left(self.start);
            self.wrapped = false;
            self.start = 0;
            self.end = 0;
        }
    }

    pub fn extract(&mut self) -> &str {
        fn is_utf8_leader(byte: u8) -> bool {
            byte & 0b10000000 == 0b00000000 || byte & 0b11100000 == 0b11000000 ||
            byte & 0b11110000 == 0b11100000 || byte & 0b11111000 == 0b11110000
        }

        self.rotate();

        let buffer = self.buffer.as_mut();
        for i in 0..buffer.len() {
            if is_utf8_leader(buffer[i]) {
                return str::from_utf8(&buffer[i..]).unwrap();
            }
        }

        ""
    }
}

impl<Storage: AsRef<[u8]> + AsMut<[u8]>> fmt::Write for LogBuffer<Storage> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            self.buffer.as_mut()[self.position] = b;
            if self.end >= self.buffer.len() - 1 {
                self.wrapped = true;
            }
            self.end = (self.end + 1) % self.buffer.as_mut().len();
        }
        Ok(())
    }
}
