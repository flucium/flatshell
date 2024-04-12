pub mod common;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "unix"))]
pub mod unix;

pub fn is_builtin(command: &str) -> bool {
    match command {
        "abort" | "cd" | "exit" => true,
        _ => false,
    }
}