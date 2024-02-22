mod common;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod darwin;

#[cfg(target_os = "linux")]
pub use linux::*;


#[cfg(target_os = "macos")]
pub use darwin::*;

pub use common::*;