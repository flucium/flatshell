use super::{token::Token, utils::replace_line_with_semicolon};

const SYMBOLS: [char; 10] = [';', '=', '\'', '"', '&', '$', '@', '|', '>', '<'];

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    is_eof: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let source = replace_line_with_semicolon(&source).chars().collect();

        Self {
            source,
            position: 0,
            is_eof: false,
        }
    }

    fn current(&self) -> Option<&char> {
        self.source.get(self.position)
    }

    fn next_ch(&mut self) -> Option<&char> {
        self.position += 1;
        self.current()
    }

    fn prev_ch(&mut self) -> Option<&char> {
        self.position -= 1;
        self.current()
    }

    // fn peek_ch(&self) -> Option<&char> {
    //     self.source.get(self.position + 1)
    // }

    fn next_while<F>(&mut self, mut f: F) -> String
    where
        F: FnMut(&char) -> bool,
    {
        let mut result = String::new();

        while let Some(c) = self.current() {
            if f(c) {
                result.push(*c);
                self.next_ch();
            } else {
                break;
            }
        }

        result
    }

    fn skip_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }

    fn read_string(&mut self) -> Option<String> {
        let is_double_quote = match self.current() {
            Some('"') => true,
            Some(_) => false,
            None => return None,
        };

        let is_single_quote = match self.current() {
            Some('\'') => true,
            Some(_) => false,
            None => return None,
        };

        if is_double_quote || is_single_quote {
            self.next_ch();
        }

        let string = self.next_while(|c| {
            if is_double_quote {
                *c != '"'
            } else if is_single_quote {
                *c != '\''
            } else {
                !c.is_whitespace() && !SYMBOLS.contains(c)
            }
        });

        if string.parse::<usize>().is_ok() {
            self.prev_ch();
            return None;
        }

        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    }

    fn read_ident(&mut self) -> Option<String> {
        match self.current() {
            Some('$') => {
                self.next_ch();
            }

            _ => return None,
        }

        let ident = self.next_while(|c| !c.is_whitespace() && !SYMBOLS.contains(c));

        if ident.is_empty() {
            self.prev_ch();
            None
        } else {
            Some(ident)
        }
    }

    fn read_number(&mut self) -> Option<usize> {
        match self.current() {
            Some(c) => {
                if c.is_digit(10) == false {
                    return None;
                }
            }
            _ => return None,
        }

        let string = self.next_while(|c| !SYMBOLS.contains(c) && !c.is_whitespace());

        match string.parse::<usize>() {
            Ok(number) => Some(number),
            Err(_) => None,
        }
    }

    fn read_fd(&mut self) -> Option<usize> {
        match self.current() {
            Some('@') => {
                self.next_ch();
            }

            _ => return None,
        }

        if let Some(fd) = self.read_number() {
            Some(fd)
        } else {
            self.prev_ch();
            None
        }
    }

    fn read_symbol(&mut self) -> Option<char> {
        let symbol = match self.current() {
            Some(c) => {
                if SYMBOLS.contains(c) {
                    *c
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        self.next_ch();

        Some(symbol)
    }

    pub fn read_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(string) = self.read_string() {
            return Token::String(string);
        }

        if let Some(number) = self.read_number() {
            return Token::Number(number);
        }

        if let Some(fd) = self.read_fd() {
            return Token::FD(fd as i32);
        }

        if let Some(ident) = self.read_ident() {
            return Token::Ident(ident);
        }

        if let Some(symbol) = self.read_symbol() {
            match symbol {
                ';' => Token::Semicolon,
                '=' => Token::Assign,
                '\'' => Token::String("'".to_string()),
                '"' => Token::String("\"".to_string()),
                '&' => Token::Ampersand,
                '$' => Token::Dollar,
                '@' => Token::String("@".to_string()),
                '|' => Token::Pipe,
                '>' => Token::Gt,
                '<' => Token::Lt,
                _ => Token::EOF,
            }
        } else {
            Token::EOF
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            tokens.push(self.read_token());

            if tokens.last() == Some(&Token::EOF) {
                break;
            }
        }

        tokens
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_token() {
            Token::EOF => {
                if self.is_eof {
                    None
                } else {
                    self.is_eof = true;
                    Some(Token::EOF)
                }
            }
            token => Some(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string() {
        assert_eq!(Lexer::new("hello").read_string(), Some("hello".to_string()));

        assert_eq!(
            Lexer::new("hello world").read_string(),
            Some("hello".to_string())
        );

        assert_eq!(
            Lexer::new("\"hello world\"").read_string(),
            Some("hello world".to_string())
        );

        assert_eq!(
            Lexer::new("\"hello = world\"").read_string(),
            Some("hello = world".to_string())
        );
    }

    #[test]
    fn read_string_none() {
        assert_eq!(Lexer::new("").read_string(), None);

        assert_eq!(Lexer::new(" ").read_string(), None);

        SYMBOLS.iter().for_each(|symbol| {
            assert_eq!(Lexer::new(symbol.to_string().as_str()).read_string(), None);
        });
    }

    #[test]
    fn read_ident() {
        assert_eq!(Lexer::new("$hello").read_ident(), Some("hello".to_string()));

        assert_eq!(
            Lexer::new("$hello world").read_ident(),
            Some("hello".to_string())
        );

        assert_eq!(
            Lexer::new("$hello = world").read_ident(),
            Some("hello".to_string())
        );

        assert_eq!(Lexer::new("$.").read_ident(), Some(".".to_string()));

        assert_eq!(Lexer::new("$0123").read_ident(), Some("0123".to_string()));
    }

    #[test]
    fn read_ident_none() {
        assert_eq!(Lexer::new("").read_ident(), None);

        assert_eq!(Lexer::new(" ").read_ident(), None);

        assert_eq!(Lexer::new("$\"hello\"").read_ident(), None);

        SYMBOLS.iter().for_each(|symbol| {
            assert_eq!(Lexer::new(symbol.to_string().as_str()).read_ident(), None);
        });
    }

    #[test]
    fn read_number() {
        assert_eq!(Lexer::new("123").read_number(), Some(123));

        assert_eq!(Lexer::new("123 ").read_number(), Some(123));

        assert_eq!(Lexer::new("123 456").read_number(), Some(123));
    }

    #[test]
    fn read_number_none() {
        assert_eq!(Lexer::new("").read_number(), None);

        assert_eq!(Lexer::new(" ").read_number(), None);

        assert_eq!(Lexer::new("123.1").read_number(), None);

        assert_eq!(Lexer::new("hello").read_number(), None);

        SYMBOLS.iter().for_each(|symbol| {
            assert_eq!(Lexer::new(symbol.to_string().as_str()).read_number(), None);
        });
    }

    #[test]
    fn read_fd() {
        assert_eq!(Lexer::new("@123").read_fd(), Some(123));

        assert_eq!(Lexer::new("@123 ").read_fd(), Some(123));

        assert_eq!(Lexer::new("@123 456").read_fd(), Some(123));

        assert_eq!(Lexer::new("@123@456").read_fd(), Some(123));

        assert_eq!(Lexer::new("@123>test.txt").read_fd(), Some(123));
    }

    #[test]
    fn read_fd_none() {
        assert_eq!(Lexer::new("").read_fd(), None);

        assert_eq!(Lexer::new(" ").read_fd(), None);

        assert_eq!(Lexer::new("@hello").read_fd(), None);

        assert_eq!(Lexer::new("@123.1").read_fd(), None);

        assert_eq!(Lexer::new("@").read_fd(), None);

        SYMBOLS.iter().for_each(|symbol| {
            assert_eq!(Lexer::new(symbol.to_string().as_str()).read_fd(), None);
        });
    }

    #[test]
    fn read_symbol() {
        SYMBOLS.iter().for_each(|symbol| {
            assert_eq!(
                Lexer::new(symbol.to_string().as_str()).read_symbol(),
                Some(*symbol)
            );
        });
    }

    #[test]
    fn read_symbol_none() {
        assert_eq!(Lexer::new("").read_symbol(), None);

        assert_eq!(Lexer::new(" ").read_symbol(), None);

        assert_eq!(Lexer::new("hello").read_symbol(), None);

        assert_eq!(Lexer::new("123").read_symbol(), None);
    }

    #[test]
    fn test_tokenize() {
        let mut lexer =
            Lexer::new("ls -a; ping -c 3 127.0.0.1 | cat -b\nls > test.txt;tee@0<test.txt");

        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::String("ls".to_string()),
                Token::String("-a".to_string()),
                Token::Semicolon,
                Token::String("ping".to_string()),
                Token::String("-c".to_string()),
                Token::Number(3),
                Token::String("127.0.0.1".to_string()),
                Token::Pipe,
                Token::String("cat".to_string()),
                Token::String("-b".to_string()),
                Token::Semicolon,
                Token::String("ls".to_string()),
                Token::Gt,
                Token::String("test.txt".to_string()),
                Token::Semicolon,
                Token::String("tee".to_string()),
                Token::FD(0),
                Token::Lt,
                Token::String("test.txt".to_string()),
                Token::EOF
            ]
        );
    }
}
