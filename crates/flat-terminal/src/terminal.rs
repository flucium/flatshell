use super::ascii;
use std::io;
use std::io::Write;
use std::process::exit;

pub const TERMINAL_BUFFER_SIZE: usize = 8192; // 8KB

pub const PROMPT_BUFFER_SIZE: usize = 32; // 32 bytes

pub struct Terminal {
    buffer: Vec<u8>,
    buffer_index: usize,
    prompt: String,
    origin_termios: libc::termios,
}

impl Terminal {
    /// Create new terminal
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(TERMINAL_BUFFER_SIZE),
            buffer_index: 0,
            prompt: String::with_capacity(PROMPT_BUFFER_SIZE),
            origin_termios: termios(),
        }
    }

    /// Set prompt
    ///
    /// # Example
    /// ```
    /// let mut t = flat_terminal::Terminal::new();
    ///
    /// t.set_prompt("#".to_owned());
    /// ```
    pub fn set_prompt(&mut self, string: &str) {
        self.prompt = string.into();
    }

    /// Read line
    ///
    /// # Example
    /// ```
    /// let mut t = flat_terminal::Terminal::new();
    ///
    /// loop{
    ///     
    ///     t.set_prompt("#");
    ///     
    ///     let string = t.read_line().unwrap();
    ///     println!("{string}");
    /// }
    /// ```
    pub fn read_line(&mut self) -> io::Result<String> {
        self.set_raw_mode();

        self.init_buffer()?;

        let mut stdout = io::BufWriter::new(io::stdout().lock());

        loop {
            stdout.flush()?;

            let ch = match getch() {
                None => continue,
                Some(ch) => ch,
            };

            match ch {
                0 => continue,

                // Ctrl + C
                3 => {
                    self.unset_raw_mode();
                    exit(0);
                }

                // Enter
                10 => break,

                // Special keys
                27 => {
                    if getch().unwrap_or(0) != 91 {
                        continue;
                    }

                    // Arrow keys
                    match getch().unwrap_or(0) {
                        //up
                        65 => {}

                        //down
                        66 => {}

                        //right
                        67 => {
                            if self.buffer_index < self.buffer.len() {
                                self.buffer_index += 1;
                                stdout.write_all(
                                    format!("{}", ascii::Cursor::Right.get_esc_code()).as_bytes(),
                                )?;
                            }
                        }

                        //left
                        68 => {
                            if self.buffer_index > 0 {
                                stdout.write_all(
                                    format!("{}", ascii::Cursor::Left.get_esc_code()).as_bytes(),
                                )?;
                                self.buffer_index -= 1;
                            }
                        }
                        _ => continue,
                    }
                }

                // Backspace
                127 => {
                    if self.buffer_index <= 0 {
                        continue;
                    }

                    self.buffer_index -= 1;

                    for i in 0..self.buffer.len() {
                        if i != 0 {
                            stdout.write_all(
                                format!("{}", ascii::Cursor::Backspace.get_esc_code()).as_bytes(),
                            )?;
                        }
                    }

                    stdout.write_all(
                        format!("\r{}{}", self.prompt, String::from_utf8_lossy(&self.buffer))
                            .as_bytes(),
                    )?;

                    self.buffer.remove(self.buffer_index);

                    stdout.write_all(
                        format!("{}", ascii::Cursor::Backspace.get_esc_code()).as_bytes(),
                    )?;

                    stdout.write_all(
                        format!(
                            "\r{}{}",
                            self.prompt,
                            String::from_utf8_lossy(&self.buffer).to_string()
                        )
                        .as_bytes(),
                    )?;

                    if self.buffer_index < self.buffer.len() {
                        let move_position = self.prompt.len() + self.buffer_index - 1;
                        stdout.write_all(
                            format!("{}", ascii::Cursor::Move(move_position).get_esc_code())
                                .as_bytes(),
                        )?;
                    }
                }

                // Insert character and print buffer
                _ => {
                    self.buffer.insert(self.buffer_index, ch);

                    self.buffer_index += 1;

                    for i in 0..self.buffer.len() {
                        if i != 0 {
                            stdout.write_all(
                                format!("{}", ascii::Cursor::Backspace.get_esc_code()).as_bytes(),
                            )?;
                        }
                    }

                    stdout.write_all(
                        format!("\r{}{}", self.prompt, String::from_utf8_lossy(&self.buffer))
                            .as_bytes(),
                    )?;

                    if self.buffer_index < self.buffer.len() {
                        let move_position = self.prompt.len() + self.buffer_index;

                        stdout.write_all(
                            format!("{}", ascii::Cursor::Move(move_position).get_esc_code())
                                .as_bytes(),
                        )?;
                    }
                }
            }
        }

        self.unset_raw_mode();

        stdout.write_all(b"\n")?;

        Ok(String::from_utf8_lossy(&self.buffer).to_string())
    }

    /// Initialize buffer
    fn init_buffer(&mut self) -> io::Result<()> {
        let mut stdout = io::BufWriter::new(io::stdout().lock());

        self.buffer.clear();

        self.buffer_index = 0;

        if self.buffer_index <= self.buffer.len() {
            let move_position = self.prompt.len() + 1;
            stdout.write_all(
                format!(
                    "\r{}{}",
                    self.prompt,
                    ascii::Cursor::Move(move_position).get_esc_code()
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }

    // Enable raw mode
    fn set_raw_mode(&mut self) {
        unsafe { libc::tcgetattr(0, &mut self.origin_termios) };

        let mut raw = self.origin_termios;

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
            libc::tcsetattr(0, 0, &self.origin_termios);
        }
    }
}

#[inline]
fn getch() -> Option<u8> {
    let code = [0; 1];

    let n = unsafe { libc::read(0, code.as_ptr() as *mut libc::c_void, 1) };

    if n <= 0 {
        return None;
    }

    Some(code[0])
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
