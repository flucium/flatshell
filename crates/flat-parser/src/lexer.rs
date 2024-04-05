use super::utils::replace_line_with_semicolon;
use crate::token::Token;
use flat_common::error::{Error, ErrorKind};
use flat_common::result::Result;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    is_eof: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let input = replace_line_with_semicolon(input).chars().collect();

        Self {
            input,
            position: 0,
            is_eof: false,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::with_capacity(self.input.len() + 1024);

        while self.is_eof == false {
            let token = self.read_token()?;

            tokens.push(token);

            if tokens.last() == Some(&Token::EOF) {
                self.is_eof = true;
                break;
            }
        }

        Ok(tokens)
    }

    fn read_token(&mut self) -> Result<Token> {
        let mut token = Token::EOF;

        while let Some(ch) = self.input.get(self.position) {
            // skip whitespace
            if ch.is_whitespace() {
                self.position += 1;
                continue;
            }

            match ch {
                ';' => {
                    token = Token::Semicolon;
                    self.position += 1;
                    break;
                }

                '|' => {
                    token = Token::Pipe;
                    self.position += 1;
                    break;
                }

                '=' => {
                    self.position += 1;
                    token = Token::Assign;
                    break;
                }

                '>' => {
                    self.position += 1;
                    token = Token::Gt;
                    break;
                }

                '<' => {
                    self.position += 1;

                    token = Token::Lt;
                    break;
                }

                '&' => {
                    self.position += 1;
                    token = Token::Ampersand;
                    break;
                }

                '$' => {
                    token = self
                        .read_ident()
                        .map(|ident| Token::Ident(ident))
                        .unwrap_or({
                            self.position += 1;
                            Token::Dollar
                        });
                    break;
                }

                '@' => {
                    token = self.read_fd().map(|fd| Token::FD(fd)).unwrap_or({
                        self.position += 1;
                        Token::String('@'.to_string())
                    });
                    break;
                }

                '0'..='9' => {
                    token = match self.read_number() {
                        Ok(number) => Token::USize(number),
                        Err(_) => self
                            .read_string()
                            .map(|string| Token::String(string))
                            .map_err(|_| Error::new(ErrorKind::LexerError, "Invalid token"))?,
                    };

                    break;
                }

                _ => {
                    token = self
                        .read_string()
                        .map(|string| Token::String(string))
                        .map_err(|_| Error::new(ErrorKind::LexerError, "Invalid token"))?;
                    break;
                }
            }
        }

        Ok(token)
    }

    fn read_string(&mut self) -> std::result::Result<String, ()> {
        let is_double_quote = self.input[self.position] == '"';

        let start = if is_double_quote {
            self.position += 1;
            self.position
        } else {
            self.position
        };

        while let Some(ch) = self.input.get(self.position) {
            self.position += 1;

            if is_double_quote == true && *ch == '"' {
                break;
            }

            if is_double_quote == false
                && (ch.is_whitespace() || matches!(ch, ';' | '|' | '>' | '<' | '&' | '$'))
            {
                self.position -= 1;
                break;
            }
        }

        let end = if is_double_quote {
            self.position - 1
        } else {
            self.position
        };

        let string = self.input[start..end].iter().collect::<String>();

        if is_double_quote == false {
            match string.parse::<isize>() {
                Ok(_) => {
                    self.position = start;
                    Err(())
                }
                Err(_) => Ok(string),
            }
        } else {
            Ok(string)
        }
    }

    fn read_ident(&mut self) -> std::result::Result<String, ()> {
        if self.input[self.position] != '$' {
            Err(())?
        }

        let start = self.position + 1;

        while let Some(ch) = self.input.get(self.position) {
            if ch.is_whitespace() {
                break;
            }
            self.position += 1;
        }

        if self.position == start {
            Err(())?
        }

        let ident = self.input[start..self.position].iter().collect::<String>();

        Ok(ident)
    }

    fn read_number(&mut self) -> std::result::Result<usize, ()> {
        let start = self.position;

        while let Some(ch) = self.input.get(self.position) {
            self.position += 1;

            if ch.is_whitespace() {
                self.position -= 1;
                break;
            }
        }

        let end = self.position;

        let number = self.input[start..end].iter().collect::<String>();

        match number.parse::<usize>() {
            Ok(number) => Ok(number),
            Err(_) => {
                self.position = start;
                Err(())
            }
        }
    }

    fn read_fd(&mut self) -> std::result::Result<i32, ()> {
        if self.input[self.position] != '@' {
            Err(())?;
        }

        let start = self.position + 1;

        while let Some(ch) = self.input.get(self.position) {
            self.position += 1;

            if ch.is_whitespace() {
                self.position -= 1;
                break;
            }
        }

        let end = self.position;

        let number = self.input[start..end]
            .iter()
            .collect::<String>()
            .parse::<i32>();

        match number {
            Ok(number) => Ok(number),
            Err(_) => {
                self.position = start - 1;
                Err(())
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof {
            return None;
        }

        match self.read_token() {
            Ok(Token::EOF) => {
                self.is_eof = true;
                None
            }
            Ok(token) => Some(token),
            Err(_) => {
                self.is_eof = true;
                Some(Token::EOF)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string() {
        // string
        assert_eq!(Lexer::new("hello").read_string(), Ok("hello".to_string()));

        // string with whitespace
        assert_eq!(
            Lexer::new("hello world").read_string(),
            Ok("hello".to_string())
        );

        // string with whitespace, first character is whitespace
        assert_eq!(Lexer::new(" hello").read_string(), Ok(String::default()));

        // string with whitespace, last character is whitespace
        assert_eq!(Lexer::new("hello ").read_string(), Ok("hello".to_string()));

        // string with whitespace, first and last character is whitespace
        assert_eq!(Lexer::new(" hello ").read_string(), Ok(String::default()));

        // string with whitespace and double quote
        assert_eq!(
            Lexer::new("\"hello world\"").read_string(),
            Ok("hello world".to_string())
        );

        // string with double quote and number
        assert_eq!(Lexer::new("\"123\"").read_string(), Ok("123".to_string()));
    }

    #[test]
    fn test_read_string_error() {
        // string with number
        assert_eq!(Lexer::new("123").read_string(), Err(()));
    }

    #[test]
    fn test_read_ident() {
        // ident
        assert_eq!(Lexer::new("$hello").read_ident(), Ok("hello".to_string()));

        // ident with whitespace
        assert_eq!(
            Lexer::new("$hello world").read_ident(),
            Ok("hello".to_string())
        );

        // ident with whitespace, last character is whitespace
        assert_eq!(Lexer::new("$hello ").read_ident(), Ok("hello".to_string()));
    }

    #[test]
    fn test_read_ident_error() {
        // ident with string
        assert_eq!(Lexer::new("hello").read_ident(), Err(()));

        // ident with whitespace, first character is whitespace
        assert_eq!(Lexer::new(" $hello").read_ident(), Err(()));

        // ident with whitespace, first and last character is whitespace
        assert_eq!(Lexer::new(" $hello ").read_ident(), Err(()));
    }

    #[test]
    fn test_read_number() {
        // number
        assert_eq!(Lexer::new("123").read_number(), Ok(123));

        // number with whitespace, last character is whitespace
        assert_eq!(Lexer::new("123 ").read_number(), Ok(123));
    }

    #[test]
    fn test_read_number_error() {
        // number with string
        assert_eq!(Lexer::new("hello").read_number(), Err(()));

        // number with whitespace, first character is whitespace
        assert_eq!(Lexer::new(" 123").read_number(), Err(()));

        // number with whitespace, first and last character is whitespace
        assert_eq!(Lexer::new(" 123 ").read_number(), Err(()));
    }

    #[test]
    fn test_read_fd() {
        // fd
        assert_eq!(Lexer::new("@123").read_fd(), Ok(123));

        // fd with whitespace
        assert_eq!(Lexer::new("@123 ").read_fd(), Ok(123));
    }

    #[test]
    fn test_read_fd_error() {
        // fd with string
        assert_eq!(Lexer::new("@hello").read_fd(), Err(()));

        // fd with whitespace, first character is whitespace
        assert_eq!(Lexer::new(" @123").read_fd(), Err(()));

        // fd with whitespace, first and last character is whitespace
        assert_eq!(Lexer::new(" @123 ").read_fd(), Err(()));
    }

    #[test]
    fn test_read_token() {
        // semicolon
        assert_eq!(Lexer::new(";").read_token(), Ok(Token::Semicolon));

        // pipe
        assert_eq!(Lexer::new("|").read_token(), Ok(Token::Pipe));

        // equal
        assert_eq!(Lexer::new("=").read_token(), Ok(Token::Assign));

        // gt
        assert_eq!(Lexer::new(">").read_token(), Ok(Token::Gt));

        // lt
        assert_eq!(Lexer::new("<").read_token(), Ok(Token::Lt));

        // ampersand
        assert_eq!(Lexer::new("&").read_token(), Ok(Token::Ampersand));

        // ident
        assert_eq!(
            Lexer::new("$hello").read_token(),
            Ok(Token::Ident("hello".to_string()))
        );

        // fd
        assert_eq!(Lexer::new("@123").read_token(), Ok(Token::FD(123)));

        // number
        assert_eq!(Lexer::new("123").read_token(), Ok(Token::USize(123)));

        // string
        assert_eq!(
            Lexer::new("hello").read_token(),
            Ok(Token::String("hello".to_string()))
        );

        // string with whitespace
        assert_eq!(
            Lexer::new("hello world").read_token(),
            Ok(Token::String("hello".to_string()))
        );

        // string with whitespace, first character is whitespace
        assert_eq!(
            Lexer::new(" hello").read_token(),
            Ok(Token::String("hello".to_string()))
        );

        // string with whitespace, last character is whitespace
        assert_eq!(
            Lexer::new("hello ").read_token(),
            Ok(Token::String("hello".to_string()))
        );

        // string with whitespace, first and last character is whitespace
        assert_eq!(
            Lexer::new(" hello ").read_token(),
            Ok(Token::String("hello".to_string()))
        );

        // input is empty
        assert_eq!(Lexer::new("").read_token(), Ok(Token::EOF));

        // input is whitespace
        assert_eq!(Lexer::new(" ").read_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_read_token_mixed() {
        let mut lexer = Lexer::new("| = > < ; $ & abcd $abcd 1234 @1");
        for token in vec![
            Token::Pipe,
            Token::Assign,
            Token::Gt,
            Token::Lt,
            Token::Semicolon,
            Token::Dollar,
            Token::Ampersand,
            Token::String("abcd".to_string()),
            Token::Ident("abcd".to_string()),
            Token::USize(1234),
            Token::FD(1),
            Token::EOF,
        ] {
            assert_eq!(lexer.read_token(), Ok(token));
        }
    }

    #[test]
    fn test_read_token_line() {
        // input line
        let mut lexer = Lexer::new("ls;ls -a | cat -b; ping -c 5 127.0.0.1:8080 > test.txt; $A");

        for token in vec![
            Token::String("ls".to_string()),
            Token::Semicolon,
            Token::String("ls".to_string()),
            Token::String("-a".to_string()),
            Token::Pipe,
            Token::String("cat".to_string()),
            Token::String("-b".to_string()),
            Token::Semicolon,
            Token::String("ping".to_string()),
            Token::String("-c".to_string()),
            Token::USize(5),
            Token::String("127.0.0.1:8080".to_string()),
            Token::Gt,
            Token::String("test.txt".to_string()),
            Token::Semicolon,
            Token::Ident("A".to_string()),
            Token::EOF,
        ] {
            assert_eq!(lexer.read_token(), Ok(token));
        }
    }

    #[test]
    fn test_iter() {
        let mut lexer = Lexer::new("ls;ls -a | cat -b; ping -c 5 127.0.0.1:8080 > test.txt; $A");

        for token in vec![
            Token::String("ls".to_string()),
            Token::Semicolon,
            Token::String("ls".to_string()),
            Token::String("-a".to_string()),
            Token::Pipe,
            Token::String("cat".to_string()),
            Token::String("-b".to_string()),
            Token::Semicolon,
            Token::String("ping".to_string()),
            Token::String("-c".to_string()),
            Token::USize(5),
            Token::String("127.0.0.1:8080".to_string()),
            Token::Gt,
            Token::String("test.txt".to_string()),
            Token::Semicolon,
            Token::Ident("A".to_string()),
            Token::EOF,
        ] {
            if let Some(t) = lexer.next() {
                assert_eq!(t, token);
            }
        }

        // After EOF is None
        assert_eq!(lexer.next(), None);

        // After EOF is None forever
        for _ in 0..100 {
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::new("ls;ls -a | cat -b; ping -c 5 127.0.0.1:8080 > test.txt; $A");

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::String("ls".to_string()),
                Token::Semicolon,
                Token::String("ls".to_string()),
                Token::String("-a".to_string()),
                Token::Pipe,
                Token::String("cat".to_string()),
                Token::String("-b".to_string()),
                Token::Semicolon,
                Token::String("ping".to_string()),
                Token::String("-c".to_string()),
                Token::USize(5),
                Token::String("127.0.0.1:8080".to_string()),
                Token::Gt,
                Token::String("test.txt".to_string()),
                Token::Semicolon,
                Token::Ident("A".to_string()),
                Token::EOF,
            ]
        );
    }
}
