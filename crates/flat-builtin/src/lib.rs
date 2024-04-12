mod common;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "unix"))]
mod unix;

pub fn is_builtin(command: &str) -> bool {
    match command {
        "abort" | "cd" | "exit" => true,
        _ => false,
    }
}

pub fn execute(command: &str, args: Vec<&str>) {
    match command {
        "abort" => {
            common::abort();
        }

        "cd" => {
            #[cfg(any(target_os = "linux", target_os = "macos", target_os = "unix"))]
            unix::cd(args.get(0).unwrap_or(&"./"));
        }

        "exit" => {
            let arg = args.get(0).unwrap_or(&"0").parse::<i32>().unwrap_or(2);

            common::exit(arg);
        }

        _ => {}
    }
}
