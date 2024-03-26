/*
    ToDo: Error
*/
use flat_common::{error::Error, result::Result};

#[derive(Debug)]
pub struct Pipe(
    Option<std::os::unix::io::RawFd>,
    Option<std::os::unix::io::RawFd>,
);

impl Pipe {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn send(&mut self, fd: std::os::unix::io::RawFd) {
        self.0 = Some(fd);
    }

    pub fn recv(&mut self) -> Result<std::os::unix::io::RawFd> {
        self.0.take().ok_or(Error::DUMMY)
    }
}