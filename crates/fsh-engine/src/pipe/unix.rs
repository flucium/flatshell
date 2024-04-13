
use std::os::unix::io::RawFd;

use fsh_common::{
    Error,
    ErrorKind,
    Result
};

#[derive(Debug, PartialEq)]
pub enum PipeState {
    Closed = 0,
    Sendable = 1,
    Recvable = 2,
}

impl PipeState {
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn from_u32(value: u32) -> Result<Self> {
        match value {
            0 => Ok(PipeState::Closed),
            1 => Ok(PipeState::Sendable),
            2 => Ok(PipeState::Recvable),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid PipeState")),
        }
    }

    pub fn from_usize(value: usize) -> Result<Self> {
        match value {
            0 => Ok(PipeState::Closed),
            1 => Ok(PipeState::Sendable),
            2 => Ok(PipeState::Recvable),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid PipeState")),
        }
    }
}

type FD = Option<std::os::unix::io::RawFd>;

#[derive(Debug, PartialEq)]
pub struct Pipe {
    state: PipeState,
    fd: FD,
}

impl Pipe {
    /// Create a new Pipe instance
    pub fn new() -> Self {
        Self {
            state: PipeState::Closed,
            fd: None,
        }
    }

    /// Open a new Pipe instance
    pub fn open() -> Self {
        Self {
            state: PipeState::Sendable,
            fd: None,
        }
    }

    /// Get the pipe state
    ///
    /// Returns the current state of the pipe.
    ///
    /// # PipeState
    /// - Closed: 0
    /// - Sendable: 1
    /// - Recvable: 2
    pub fn state(&self) -> &PipeState {
        &self.state
    }

    /// Get the file descriptor
    ///
    /// Returns the value of std::os::unix::io::RawFd. if the pipe's FD is not set, that is, None, returns None.
    pub fn fd(&self) -> Option<&RawFd> {
        self.fd.as_ref()
    }

    /// Close the Pipe instance
    ///
    /// FD is closed only when FD is greater than or equal to 0. In other words, it is not closed when FD is a negative number.
    ///
    /// If you want to close all FDs regardless of their value, use `pub fn quit`.
    ///
    /// # Errors
    /// FD is less than 0, return error.
    pub fn close(&mut self) -> Result<()> {
        // if FD is Some and FD is greater than or equal to 0,
        //  use libc::close to close the FD.
        if let Some(fd) = self.fd {
            if fd >= 0 {
                unsafe { libc::close(fd) };
            } else {
                Err(Error::new(ErrorKind::BrokenPipe, "Pipe is broken"))?
            }
        }

        // Reset the state and file descriptor
        // State: Closed
        // FD: None
        self.state = PipeState::Closed;
        self.fd = None;
        Ok(())
    }

    /// Quit the Pipe instance
    ///
    /// In most cases, it works the same as `pub fn close`. The difference is that it forcibly closes the FD regardless of its value.
    ///
    /// How to use it with `pub fn close`: close operates on the assumption that FD is greater than or equal to 0, and will generate an error if fd < 0.
    /// however, quit will close the FD even if fd is a negative number. Therefore, the Pipe and captured FD are always closed, so it can be safely used with `ManuallyDrop` etc.
    pub fn quit(&mut self) {
        if let Some(fd) = self.fd.take() {
            unsafe { libc::close(fd) };
        }

        self.state = PipeState::Closed;
        self.fd = None;
    }

    /// Cancel the Pipe state
    ///
    /// If FD is Some, it is forcibly Closed regardless of the FD value. Then, the Pipe is reset to a transmittable state.
    pub fn cancel(mut self) {
        if let Some(fd) = self.fd {
            unsafe { libc::close(fd) };
        }

        self.state = PipeState::Sendable;
        self.fd = None;
    }
    
    /// Check if the Pipe instance is sendable
    pub fn is_sendable(&self) -> bool {
        self.state == PipeState::Sendable
    }

    /// Check if the Pipe instance is recvable
    pub fn is_recvable(&self) -> bool {
        self.state == PipeState::Recvable
    }

    /// Send the file descriptor to the Pipe
    ///
    /// If the state is Sendable, the file descriptor is set to the Pipe instance.
    ///
    /// # Arguments
    /// - `fd`: File descriptor
    ///
    /// # Errors
    /// - Pipe is broken: If the File descriptor is Some.
    /// - Pipe is closed: If the Pipe is closed.
    /// - Pipe is in recvable state: If the Pipe is in recvable state.
    pub fn send(&mut self, fd: std::os::unix::io::RawFd) -> Result<()> {
        // if the state is Sendable, fd is None.
        if self.fd.is_some() {
            Err(Error::new(ErrorKind::BrokenPipe, "Pipe is broken"))?
        }

        match self.state {
            PipeState::Sendable => {
                self.fd = Some(fd);
                self.state = PipeState::Recvable;
                Ok(())
            }

            PipeState::Closed => Err(Error::new(ErrorKind::BrokenPipe, "Pipe is closed")),

            PipeState::Recvable => Err(Error::new(
                ErrorKind::BrokenPipe,
                "Pipe is in recvable state",
            )),
        }
    }

    /// Receive the file descriptor from the Pipe
    ///
    /// You can receive the sent FD. For example, if you send FD 3 using send, recv will return FD 3.
    ///
    /// # Errors
    /// - Pipe is broken: If the File descriptor is None.
    /// - Pipe is closed: If the Pipe is closed.
    /// - Pipe is in sendable state: If the Pipe is in sendable state.
    pub fn recv(&mut self) -> Result<std::os::unix::io::RawFd> {
        // if the state is Recvable, fd is Some.
        if self.fd.is_none() {
            Err(Error::new(ErrorKind::BrokenPipe, "Pipe is broken"))?
        }

        match self.state {
            PipeState::Recvable => {
                // get fd
                let fd = self.fd.unwrap_or(-1);

                // fd is less than 0, return error
                if fd < 0 {
                    Err(Error::new(ErrorKind::BrokenPipe, "Pipe is broken"))?
                }

                // reset the state and file descriptor
                self.state = PipeState::Sendable;
                self.fd = None;

                Ok(fd)
            }

            PipeState::Closed => Err(Error::new(ErrorKind::BrokenPipe, "Pipe is closed")),

            PipeState::Sendable => Err(Error::new(
                ErrorKind::BrokenPipe,
                "Pipe is in sendable state",
            )),
        }
    }
}