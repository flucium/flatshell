pub mod common;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "unix"))]
pub mod unix;