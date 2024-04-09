#[derive(Debug)]
pub struct Pipe(u32, std::os::unix::io::RawFd);

impl Pipe {
    pub fn new() -> Self {
        Self(0, 0)
    }

    pub fn open() -> Self {
        Self(1, 0)
    }

    pub fn close(&mut self) {
        self.0 = 0;
    }

    pub fn is_sendable(&self) -> bool {
        self.0 == 1
    }

    pub fn is_recvable(&self) -> bool {
        self.0 == 2
    }

    pub fn send(&mut self, fd: std::os::unix::io::RawFd) {
        if self.0 == 1 {
            self.1 = fd;
            self.0 = 2;
        } else {
            panic!("Pipe is not sendable");
        }
    }

    pub fn recv(&mut self) -> std::os::unix::io::RawFd {
        if self.0 == 2 {
            self.0 = 1;
            self.1
        } else {
            panic!("Pipe is not recvable");
        }
    }
}
