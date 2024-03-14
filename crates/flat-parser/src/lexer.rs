use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    is_eof: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            is_eof: false,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next() {
            tokens.push(token);
        }

        tokens
    }

    fn read(&mut self) -> Token {
        let mut tkn = Token::EOF;

        while let Some(ch) = self.input.get(self.position) {
            if ch.is_whitespace() {
                self.position += 1;
                continue;
            }

            match ch {
                ';' => {
                    self.position += 1;
                    tkn = Token::Semicolon;
                    break;
                }

                '|' => {
                    self.position += 1;
                    tkn = Token::Pipe;
                    break;
                }

                '=' => {
                    self.position += 1;
                    tkn = Token::Assign;
                    break;
                }

                '>' => {
                    self.position += 1;
                    tkn = Token::Gt;
                    break;
                }

                '<' => {
                    self.position += 1;

                    tkn = Token::Lt;
                    break;
                }

                '&' => {
                    self.position += 1;
                    tkn = Token::Ampersand;
                    break;
                }

                '$' => {
                    self.position += 1;

                    if let Some(peek_ch) = self.input.get(self.position) {
                        if peek_ch.is_whitespace() == false {
                            tkn = Token::Ident(self.read_string());
                            self.position += 1;
                            break;
                        }
                    }

                    tkn = Token::Dollar;

                    break;
                }

                '@' => {
                    let origin = self.position;

                    if let Some(peek_ch) = self.input.get(self.position + 1) {
                        if peek_ch.is_whitespace() == false {
                            self.position += 1;

                            let string = self.read_string();

                            if let Ok(n) = string.parse::<i32>() {
                                tkn = Token::FD(n);
                            } else {
                                self.position = origin;
                            }

                            break;
                        }
                    }

                    tkn = Token::String("@".to_string());
                }

                '0'..='9' => {
                    let string = self.read_string();

                    if let Ok(n) = string.parse::<usize>() {
                        tkn = Token::USize(n);
                    } else {
                        tkn = Token::String(string);
                    }

                    break;
                }

                _ => {
                    tkn = Token::String(self.read_string());
                    break;
                }
            }
        }

        tkn
    }

    fn read_string(&mut self) -> String {
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

        self.input[start..end].iter().collect()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tkn = self.read();

        if tkn == Token::EOF {
            if self.is_eof {
                None
            } else {
                self.is_eof = true;
                Some(tkn)
            }
        } else {
            Some(tkn)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "| = > < ; $ & abcd $abcd 1234 @1 @-1";

        let mut lexer = Lexer::new(input);

        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
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
                Token::FD(-1),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_exing() {
        let input = "echo hello world;ping -c 3 github.com | cat -b";

        let mut lexer = Lexer::new(input);

        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::String("echo".to_string()),
                Token::String("hello".to_string()),
                Token::String("world".to_string()),
                Token::Semicolon,
                Token::String("ping".to_string()),
                Token::String("-c".to_string()),
                Token::USize(3),
                Token::String("github.com".to_string()),
                Token::Pipe,
                Token::String("cat".to_string()),
                Token::String("-b".to_string()),
                Token::EOF
            ]
        );
    }
}
