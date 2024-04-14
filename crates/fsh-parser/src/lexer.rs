use super::{
    token::Token,
    utils::{remove_comment, remove_empty_line, replace_line_with_semicolon},
};

const SYMBOLS: [char; 11] = [';', '=', '\\', '\'', '"', '&', '$', '@', '|', '>', '<'];

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    /// Create a new lexer.
    pub fn new(source: &str) -> Self {
        let preprocess = |source: &str| -> String {
            let p1 = remove_comment(source);

            let p2 = remove_empty_line(&p1);

            let p3 = replace_line_with_semicolon(&p2);

            p3
        };

        let source = preprocess(source);

        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<&char> {
        self.source.get(self.position)
    }

    fn peek_char(&self) -> Option<&char> {
        self.source.get(self.position + 1)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_while<F>(&mut self, f: F) -> (Vec<char>, Option<&char>)
    where
        F: Fn(char) -> bool,
    {
        let mut result = Vec::new();

        while let Some(&c) = self.source.get(self.position) {
            if f(c) {
                result.push(c);
                self.position += 1;
            } else {
                break;
            }
        }

        let next = self.source.get(self.position);

        (result, next)
    }

    fn read_string(&mut self) -> std::result::Result<Option<String>, String> {
        let current_char = self.current_char();

        if current_char.is_none() {
            return Ok(None);
        }

        let start_position = self.position;

        let is_double_quote = current_char == Some(&'"');

        let is_single_quote = current_char == Some(&'\'');

        if is_double_quote || is_single_quote {
            self.advance();
        }

        let (string, end_char) = self.read_while(|c| {
            if is_double_quote {
                c != '"'
            } else if is_single_quote {
                c != '\''
            } else {
                !SYMBOLS.contains(&c) && !c.is_whitespace()
            }
        });

        if is_double_quote && end_char != Some(&'"') {
            self.position = start_position;

            Err("double quote error".to_string())?
        } else if is_single_quote && end_char != Some(&'\'') {
            self.position = start_position;

            Err("single quote error".to_string())?
        } else if !is_double_quote && !is_single_quote && string.is_empty() {
            self.position = start_position;

            return Ok(None);
        } else {
            if is_double_quote || is_single_quote {
                self.advance();
            }
        }

        Ok(Some(string.into_iter().collect()))
    }

    fn read_number(&mut self) -> std::result::Result<Option<usize>, String> {
        match self.current_char() {
            Some(c) => {
                if !c.is_digit(10) {
                    Err("invalid number".to_string())?
                }
            }
            None => return Ok(None),
        }

        let start_position = self.position;

        let (string, _) = self.read_while(|c| !c.is_whitespace() && !SYMBOLS.contains(&c));

        match string.into_iter().collect::<String>().parse::<usize>() {
            Ok(number) => Ok(Some(number)),
            Err(_) => {
                self.position = start_position;
                Err("invalid number".to_string())
            }
        }
    }

    fn read_ident(&mut self) -> std::result::Result<Option<String>, String> {
        let current_char = self.current_char();

        if current_char.is_none() {
            return Ok(None);
        }

        let start_position = self.position;

        if current_char == Some(&'$') {
            self.advance();
        } else {
            self.position = start_position;
            Err("invalid identifier".to_string())?
        }

        let (string, _) = self.read_while(|c| !c.is_whitespace() && !SYMBOLS.contains(&c));

        if string.is_empty() {
            self.position = start_position;
            Err("invalid identifier".to_string())?
        }

        if let Some(ch) = string.first() {
            if !ch.is_alphabetic() {
                self.position = start_position;
                Err("invalid identifier".to_string())?
            }
        } else {
            self.position = start_position;
            Err("invalid identifier".to_string())?
        }

        if let Some(c) = string.last() {
            if !c.is_alphanumeric() {
                self.position = start_position;
                Err("invalid identifier".to_string())?
            }
        } else {
            self.position = start_position;
            Err("invalid identifier".to_string())?
        }

        Ok(Some(string.into_iter().collect()))
    }

    fn read_fd(&mut self) -> std::result::Result<Option<usize>, String> {
        let current_char = self.current_char();

        if current_char.is_none() {
            return Ok(None);
        }

        let start_position = self.position;

        if current_char == Some(&'@') {
            self.advance();
        } else {
            self.position = start_position;
            Err("invalid file descriptor".to_string())?
        }

        let (string, _) = self.read_while(|c| !c.is_whitespace() && !SYMBOLS.contains(&c));

        match string.into_iter().collect::<String>().parse::<usize>() {
            Ok(number) => Ok(Some(number)),
            Err(_) => {
                self.position = start_position;
                Err("invalid file descriptor".to_string())
            }
        }
    }

    fn read(&mut self) -> fsh_common::Result<Token> {
        let mut token = Token::EOF;

        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            match ch {
                ';' => {
                    token = Token::Semicolon;
                    self.advance();
                    break;
                }

                '=' => {
                    token = Token::Assign;
                    self.advance();
                    break;
                }

                '&' => {
                    token = Token::Ampersand;
                    self.advance();
                    break;
                }

                '|' => {
                    token = Token::Pipe;
                    self.advance();
                    break;
                }

                '>' => {
                    token = Token::Gt;
                    self.advance();
                    break;
                }

                '<' => {
                    token = Token::Lt;
                    self.advance();
                    break;
                }

                '@' => {
                    // if let Ok(fd) = self.read_fd() {
                    //     if let Some(fd) = fd {
                    //         token = Token::FD(fd as i32);
                    //     } else {
                    //         token = Token::EOF;
                    //     }
                    // } else {
                    //     token = Token::String("@".to_string());
                    //     self.advance();
                    // }

                    match self.read_fd() {
                        Ok(Some(fd)) => token = Token::FD(fd as i32),
                        Ok(None) => token = Token::EOF,
                        Err(err) => match self.peek_char() {
                            Some(ch) => {
                                if ch.is_whitespace() {
                                    token = Token::String("@".to_string());
                                    self.advance();
                                } else {
                                    Err(fsh_common::Error::new(fsh_common::ErrorKind::LexerError, &err))?;
                                }
                            }
                            None => {
                                token = Token::String("@".to_string());
                                self.advance();
                            }
                        },
                    }

                    break;
                }

                '$' => {
                    // if let Ok(ident) = self.read_ident() {
                    //     if let Some(ident) = ident {
                    //         token = Token::Ident(ident);
                    //     } else {
                    //         token = Token::EOF;
                    //     }
                    // } else {
                    //     token = Token::String("$".to_string());
                    //     self.advance();
                    // }

                    match self.read_ident() {
                        Ok(Some(ident)) => token = Token::Ident(ident),
                        Ok(None) => token = Token::EOF,
                        Err(err) => match self.peek_char() {
                            Some(ch) => {
                                if ch.is_whitespace() {
                                    token = Token::String("$".to_string());
                                    self.advance();
                                } else {
                                    Err(fsh_common::Error::new(fsh_common::ErrorKind::LexerError, &err))?;
                                }
                            }
                            None => {
                                token = Token::String("$".to_string());
                                self.advance();
                            }
                        },
                    }

                    break;
                }

                '"' | '\'' => {
                    // if let Ok(string) = self.read_string() {
                    //     if let Some(string) = string {
                    //         token = Token::String(string);
                    //     } else {
                    //         token = Token::EOF;
                    //     }
                    // }

                    match self.read_string() {
                        Ok(Some(string)) => token = Token::String(string),
                        Ok(None) => token = Token::EOF,
                        Err(err) => Err(fsh_common::Error::new(fsh_common::ErrorKind::LexerError, &err))?,
                    }

                    break;
                }

                '0'..='9' => {
                    // if let Ok(number) = self.read_number() {
                    //     if let Some(number) = number {
                    //         token = Token::Number(number);
                    //     } else {
                    //         token = Token::EOF;
                    //     }
                    // }

                    match self.read_number() {
                        Ok(Some(number)) => token = Token::Number(number),
                        Ok(None) => token = Token::EOF,
                        Err(err) => match self.read_string() {
                            Ok(Some(string)) => token = Token::String(string),
                            Ok(None) => token = Token::EOF,
                            Err(_) => Err(fsh_common::Error::new(fsh_common::ErrorKind::LexerError, &err))?,
                        },
                    }

                    break;
                }

                _ => {
                    // if let Ok(string) = self.read_string() {
                    //     if let Some(string) = string {
                    //         token = Token::String(string);
                    //     } else {
                    //         token = Token::EOF;
                    //     }
                    // }

                    match self.read_string() {
                        Ok(Some(string)) => token = Token::String(string),
                        Ok(None) => token = Token::EOF,
                        Err(err) => {
                            if self.peek_char().is_some() {
                                Err(fsh_common::Error::new(fsh_common::ErrorKind::LexerError, &err))?;
                            } else {
                                token = Token::EOF;
                            }
                        }
                    }

                    break;
                }
            }
        }

        Ok(token)
    }

    pub fn tokenize(&mut self) -> fsh_common::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.read()?;

            if token == Token::Semicolon {
                if tokens.last() == Some(&Token::Semicolon) {
                    continue;
                }
            }

            tokens.push(token);

            if tokens.last() == Some(&Token::EOF) {
                break;
            }
        }

        Ok(tokens)
    }
}