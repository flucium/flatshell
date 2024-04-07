use std::{env, path};

// User name
const USER_NAME: &str = "\\u";

// shell name
const SHELL_NAME: &str = "\\s";

// Shell full name
const SHELL_NAME_FULL: &str = "\\S";

// Shell version
const SHELL_VERSION: &str = "\\v";

// Current directory name
const CURRENT_DIRECTORY: &str = "\\w";

// Current directory full path
const CURRENT_DIRECTORY_FULL: &str = "\\W";

/// Get user name from environment variable
fn get_user_name() -> String {
    env::var("USER").unwrap_or_default()
}

/// Get current directory name from environment variable
fn get_current_directory() -> String {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Get current directory full path from environment variable
/// This function is not used in the code
fn get_current_directory_full() -> String {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .to_string_lossy()
        .to_string()
}

fn decode(source: impl Into<String>) -> String {
    let mut string = String::from(source.into());

    if string.contains(USER_NAME) {
        string = string.replace(USER_NAME, &get_user_name());
    }

    if string.contains(SHELL_NAME) {
        string = string.replace(SHELL_NAME, "fsh");
    }

    if string.contains(SHELL_NAME_FULL) {
        string = string.replace(SHELL_NAME_FULL, "FlatShell");
    }

    if string.contains(SHELL_VERSION) {
        string = string.replace(SHELL_VERSION, "0.0.1");
    }

    if string.contains(CURRENT_DIRECTORY) {
        string = string.replace(CURRENT_DIRECTORY, &get_current_directory());
    }

    if string.contains(CURRENT_DIRECTORY_FULL) {
        string = string.replace(CURRENT_DIRECTORY_FULL, &get_current_directory_full());
    }

    string
}

pub fn prompt(source: impl Into<String>) -> String {
    decode(source.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_name() {
        assert_eq!(get_user_name(), env::var("USER").unwrap_or_default());
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(USER_NAME), get_user_name());

        assert_eq!(decode(SHELL_NAME), "fsh");

        assert_eq!(decode(SHELL_NAME_FULL), "FlatShell");

        assert_eq!(decode(SHELL_VERSION), "0.0.1");

        assert_eq!(
            decode(CURRENT_DIRECTORY),
            env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
        );

        assert_eq!(
            decode(CURRENT_DIRECTORY_FULL),
            env::current_dir().unwrap().to_string_lossy()
        );
    }

    #[test]
    fn test_prompt() {
        assert_eq!(prompt(USER_NAME), get_user_name());

        assert_eq!(prompt(SHELL_NAME), "fsh");

        assert_eq!(prompt(SHELL_NAME_FULL), "FlatShell");

        assert_eq!(prompt(SHELL_VERSION), "0.0.1");

        assert_eq!(
            prompt(CURRENT_DIRECTORY),
            env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
        );

        assert_eq!(
            prompt(CURRENT_DIRECTORY_FULL),
            env::current_dir().unwrap().to_string_lossy()
        );
    }

    #[test]
    fn test_prompt_with_custom() {
        assert_eq!(prompt("Hello, \\u"), format!("Hello, {}", get_user_name()));

        assert_eq!(prompt("ShellName: \\s"), "ShellName: fsh");

        assert_eq!(prompt("ShellFullName: \\S"), "ShellFullName: FlatShell");

        assert_eq!(prompt("ShellVersion: \\v"), "ShellVersion: 0.0.1");

        assert_eq!(
            prompt("CurrentDirectory: \\w"),
            format!(
                "CurrentDirectory: {}",
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            )
        );

        assert_eq!(
            prompt("CurrentFullDirectory: \\W"),
            format!(
                "CurrentFullDirectory: {}",
                env::current_dir().unwrap().to_string_lossy()
            )
        );
    }

    #[test]
    fn test_prompt_realcase() {
        assert_eq!(
            prompt("\\u@\\w $ "),
            format!(
                "{}@{} $ ",
                get_user_name(),
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            )
        );

        assert_eq!(
            prompt("\\u@\\W $ "),
            format!(
                "{}@{} $ ",
                get_user_name(),
                env::current_dir().unwrap().to_string_lossy()
            )
        );

        assert_eq!(
            prompt("\\s \\u@\\w $ "),
            format!(
                "fsh {}@{} $ ",
                get_user_name(),
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            )
        );

        assert_eq!(
            prompt("\\S \\u@\\w $ "),
            format!(
                "FlatShell {}@{} $ ",
                get_user_name(),
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            )
        );

        assert_eq!(
            prompt("\\s(\\v) \\u@\\w $ "),
            format!(
                "fsh(0.0.1) {}@{} $ ",
                get_user_name(),
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            )
        );
    }
}
