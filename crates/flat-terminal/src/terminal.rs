use super::ascii;
use super::line::*;
use super::History;
use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};

use libc::exit;
use std::io;
use std::io::Write;

pub struct Terminal {
    termios: libc::termios,
    prompt: String,
    history: Option<History>,
}

impl Terminal {

    /// Create a new terminal
    /// 
    /// # Example
    /// ```
    /// use flat_terminal::Terminal;
    /// 
    /// let mut terminal = Terminal::new();
    /// 
    /// terminal.set_prompt("> ");
    /// 
    /// // History is disabled if there is no history.
    /// terminal.set_history(History::new());
    /// ```
    pub fn new() -> Self {
        Self {
            termios: termios(),
            prompt: String::new(),
            history: None,
        }
    }

    /// Set the prompt
    pub fn set_prompt(&mut self, prompt: impl Into<String>) {
        self.prompt = prompt.into();
    }

    /// Set the history
    /// 
    /// History is disabled if there is no history.
    pub fn set_history(&mut self, history: History) {
        self.history = Some(history);
    }

    /// Read a line
    /// 
    /// # Example
    /// ```
    /// use flat_terminal::Terminal;
    /// 
    /// let mut terminal = Terminal::new();
    /// 
    /// terminal.set_prompt("> ");
    /// 
    /// let line = terminal.read_line();
    /// ```
    pub fn read_line(&mut self) -> Result<String> {
        self.set_raw_mode();

        let mut stdout = io::stdout().lock();

        let mut line = Line::new();

        stdout
            .write_all(format!("{}", self.prompt).as_bytes())
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to write to stdout"))?;

        loop {
            stdout
                .flush()
                .map_err(|_| Error::new(ErrorKind::Other, "Failed to flush stdout"))?;

            let ch = match get_char() {
                Some(ch) => ch,
                None => continue,
            };

            match ch {
                3 => {
                    stdout.write_all(b"\n").map_err(|_| {
                        Error::new(ErrorKind::Other, "Failed to write to stdout")
                    })?;
                    self.unset_raw_mode();
                    unsafe { exit(0) };
                }

                // Enter
                10 => {
                    break;
                }

                27 => {
                    if get_char().unwrap_or(0) != 91 {
                        continue;
                    }

                    // Arrow keys
                    match get_char().unwrap_or(0) {
                        // Up
                        65 => {
                            if let Some(history) = self.history.as_mut() {
                                if let Some(entry) = history.prev() {
                                    line.clear();

                                    for b in entry.as_bytes() {
                                        line.insert(*b);
                                    }

                                    stdout
                                        .write_all(
                                            format!("{}", ascii::Cursor::ClearLine.get_esc_code())
                                                .as_bytes(),
                                        )
                                        .map_err(|_| {
                                            Error::new(
                                                ErrorKind::Other,
                                                "Failed to write to stdout",
                                            )
                                        })?;

                                    stdout
                                        .write_all(
                                            format!("\r{}{}", self.prompt, line.to_string())
                                                .as_bytes(),
                                        )
                                        .map_err(|_| {
                                            Error::new(
                                                ErrorKind::Other,
                                                "Failed to write to stdout",
                                            )
                                        })?;
                                }
                            }
                        }

                        // Down
                        66 => {
                            if let Some(history) = self.history.as_mut() {
                                if let Some(entry) = history.next() {
                                    line.clear();

                                    for b in entry.as_bytes() {
                                        line.insert(*b);
                                    }

                                    stdout
                                        .write_all(
                                            format!("{}", ascii::Cursor::ClearLine.get_esc_code())
                                                .as_bytes(),
                                        )
                                        .map_err(|_| {
                                            Error::new(
                                                ErrorKind::Other,
                                                "Failed to write to stdout",
                                            )
                                        })?;

                                    stdout
                                        .write_all(
                                            format!("\r{}{}", self.prompt, line.to_string())
                                                .as_bytes(),
                                        )
                                        .map_err(|_| {
                                            Error::new(
                                                ErrorKind::Other,
                                                "Failed to write to stdout",
                                            )
                                        })?;
                                }
                            }
                        }

                        // Right
                        67 => {
                            if line.position() < line.len() {
                                line.right();

                                stdout
                                    .write_all(
                                        format!("{}", ascii::Cursor::Right.get_esc_code())
                                            .as_bytes(),
                                    )
                                    .map_err(|_| {
                                        Error::new(ErrorKind::Other, "Failed to write to stdout")
                                    })?;
                            }
                        }

                        // Left
                        68 => {
                            if line.position() > 0 {
                                stdout
                                    .write_all(
                                        format!("{}", ascii::Cursor::Left.get_esc_code())
                                            .as_bytes(),
                                    )
                                    .map_err(|_| {
                                        Error::new(ErrorKind::Other, "Failed to write to stdout")
                                    })?;

                                line.left();
                            }
                        }

                        _ => continue,
                    }
                }

                // Backspace
                127 => {
                    if line.position() <= 0 {
                        continue;
                    }

                    for i in 0..line.len() {
                        if i != 0 {
                            stdout
                                .write_all(
                                    format!("{}", ascii::Cursor::Backspace.get_esc_code())
                                        .as_bytes(),
                                )
                                .map_err(|_| {
                                    Error::new(ErrorKind::Other, "Failed to write to stdout")
                                })?;
                        }
                    }

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Other, "Failed to write to stdout"))?;

                    line.backspace();

                    stdout
                        .write_all(
                            format!("{}", ascii::Cursor::Backspace.get_esc_code()).as_bytes(),
                        )
                        .map_err(|_| Error::new(ErrorKind::Other, "Failed to write to stdout"))?;

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string(),).as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Other, "Failed to write to stdout"))?;

                    if line.position() < line.len() {
                        let move_position = self.prompt.len() + line.position() - 1;

                        stdout
                            .write_all(
                                format!("{}", ascii::Cursor::Move(move_position).get_esc_code())
                                    .as_bytes(),
                            )
                            .map_err(|_| {
                                Error::new(ErrorKind::Other, "Failed to write to stdout")
                            })?;
                    }
                }

                _ => {
                    line.insert(ch);

                    for i in 0..line.len() {
                        if i != 0 {
                            stdout
                                .write_all(
                                    format!("{}", ascii::Cursor::Backspace.get_esc_code())
                                        .as_bytes(),
                                )
                                .map_err(|_| {
                                    Error::new(ErrorKind::Other, "Failed to write to stdout")
                                })?;
                        }
                    }

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Other, "Failed to write to stdout"))?;

                    if line.position() < line.len() {
                        let move_position = line.len() + line.position();

                        stdout
                            .write_all(
                                format!("{}", ascii::Cursor::Move(move_position).get_esc_code())
                                    .as_bytes(),
                            )
                            .map_err(|_| {
                                Error::new(ErrorKind::Other, "Failed to write to stdout")
                            })?;
                    }
                }
            }
        }

        self.unset_raw_mode();

        if let Some(history) = self.history.as_mut() {
            history.push(line.to_string());
        }

        stdout
            .write_all(b"\n")
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to write to stdout"))?;

        stdout
            .flush()
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to flush stdout"))?;

        let line = line.to_string();

        Ok(line)
    }

    // Enable raw mode
    fn set_raw_mode(&mut self) {
        unsafe { libc::tcgetattr(0, &mut self.termios) };

        let mut raw = self.termios;

        raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO | libc::IEXTEN | libc::ISIG);
        // raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO );

        raw.c_cc[libc::VTIME] = 0;

        raw.c_cc[libc::VMIN] = 1;

        unsafe {
            libc::tcsetattr(0, 0, &raw);
            libc::fcntl(0, libc::F_SETFL);
        }
    }

    // Disable raw mode
    fn unset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.termios);
        }
    }
}

#[inline]
fn get_char() -> Option<u8> {
    let code = [0; 1];

    let n = unsafe { libc::read(0, code.as_ptr() as *mut libc::c_void, 1) };

    if n <= 0 {
        return None;
    }

    Some(code[0])
}

#[inline]
#[cfg(target_os = "linux")]
fn termios() -> libc::termios {
    libc::termios {
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
    }
}

#[inline]
#[cfg(target_os = "macos")]
fn termios() -> libc::termios {
    libc::termios {
        c_cc: [0u8; 20],
        c_ispeed: 0,
        c_ospeed: 0,
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
    }
}
