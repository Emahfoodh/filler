use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct Logger {
    file: std::fs::File,
}

impl Logger {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Logger { file })
    }

    pub fn log(&mut self, message: &str) -> io::Result<()> {
        writeln!(self.file, "{}", message)
    }
}
