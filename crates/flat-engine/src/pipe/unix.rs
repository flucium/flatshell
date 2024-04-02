use std::f32::consts::E;

/*
    ToDo: Error
*/
use flat_common::{error::Error, result::Result};

// #[derive(Debug)]
// pub struct Pipe(
//     Option<std::os::unix::io::RawFd>,
//     Option<std::os::unix::io::RawFd>,
// );

// impl Pipe {
//     /// Create a new pipe
//     pub const fn new() -> Self {
//         Self(None, None)
//     }

//     /// Send data to the pipe
//     pub fn send(&mut self, fd: std::os::unix::io::RawFd) {
//         self.0 = Some(fd);
//     }

//     /// Receive data from the pipe
//     pub fn recv(&mut self) -> Result<std::os::unix::io::RawFd> {
//         self.0.take().ok_or(Error::DUMMY)
//     }
// }

// #[derive(Debug)]
// pub struct Pipe(pub u32, std::os::unix::io::RawFd);

// impl Pipe {
//     pub fn new() -> Self {
//         Self(0, -1)
//     }

//     pub fn open() -> Self {
//         Self(1, -1)
//     }

//     pub fn close(&mut self) {
//         self.0 = 0;
//     }

//     pub fn is_closed(&self) -> bool {
//         self.0 == 0
//     }

//     pub fn is_sendable(&self) -> bool {
//         if self.0 == 1 {
//             true
//         } else {
//             false
//         }
//     }

//     pub fn is_recvable(&self) -> bool {
//         if self.0 == 2 {
//             true
//         } else {
//             false
//         }
//     }

//     pub fn send(&mut self, fd: std::os::unix::io::RawFd) -> Result<()> {
//         if self.0 != 1 {
//             Err(Error::DUMMY)?;
//         }


//         self.1 = fd;
//         self.0 = 2;

//         Ok(())
//     }

//     pub fn recv(&mut self) -> Result<std::os::unix::io::RawFd> {
//         if self.0 != 2 {
//             Err(Error::DUMMY)?;
//         }

        
//         // self.0 = 1;
//         let fd = self.1;

//         Ok(fd)
//     }
// }
