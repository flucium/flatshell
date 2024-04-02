// use flat_common::{error::Error, result::Result};
// #[cfg(any(target_os = "linux", target_os = "macos"))]
// mod unix;

// #[cfg(any(target_os = "linux", target_os = "macos"))]
// pub use unix::Pipe;



// /// ThreadPipe is multi platform
// #[derive(Debug)]
// pub struct ThreadPipe(
//     std::sync::mpsc::Sender<std::process::Stdio>,
//     std::sync::mpsc::Receiver<std::process::Stdio>,
// );

// impl ThreadPipe {
//     /// Create a new thread pipe
//     pub fn new() -> Self {
//         let (tx, rx) = std::sync::mpsc::channel();

//         Self(tx, rx)
//     }

//     /// Send a stdio
//     pub fn send(&self, stdio: std::process::Stdio) -> Result<()> {
//         self.0.send(stdio).map_err(|_| Error::DUMMY)
//     }

//     /// Receive a stdio
//     pub fn recv(&self) -> Result<std::process::Stdio> {
//         self.1.recv().map_err(|_| Error::DUMMY)
//     }
// }
