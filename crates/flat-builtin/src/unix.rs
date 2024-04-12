use std::env::{set_current_dir, set_var};
use std::io::{stderr, Write};
use std::path::Path;

pub fn cd(path: &str) {
    let path = Path::new(path);

    let path = match path.exists() {
        true => path,
        false => Path::new("/"),
    };

    if set_current_dir(path).is_err() {
        if stderr()
            .lock()
            .write_all(b"cd: no such file or directory\n")
            .is_err()
        {
            panic!("internal error: cannot write to stderr")
        }
    }

    set_var("PWD", path);
}
