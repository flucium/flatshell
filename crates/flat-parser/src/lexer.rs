use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    is_eof:bool,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            is_eof: false,
        }
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
            if self.is_eof{
                None
            }else{
                self.is_eof = true;
                Some(tkn)
            }
        } else {
            Some(tkn)
        }
    }
}