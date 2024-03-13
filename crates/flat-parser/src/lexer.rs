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

    /*
        Test lexer with pseudo command
    */

    #[test]
    fn test_lexer_pseudo_command() {
        let input = "ls -a ./";

        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::String("ls".to_string()),
            Token::String("-a".to_string()),
            Token::String("./".to_string()),
            Token::EOF,
        ];

        for tkn in expected {
            assert_eq!(lexer.next(), Some(tkn));
        }
    }

    /*
        Test lexer with semicolon
        - test_lexer_semicolon
        - test_lexer_semicolon_with_space
    */

    #[test]
    fn test_lexer_semicolon() {
        let input = ";";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Semicolon);
    }

    #[test]
    fn test_lexer_semicolon_with_space() {
        let input = " ; ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Semicolon);

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    /*
        Test lexer with pipe
        - test_lexer_pipe
        - test_lexer_pipe_with_space
    */

    #[test]
    fn test_lexer_pipe() {
        let input = "|";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Pipe);
    }

    #[test]
    fn test_lexer_pipe_with_space() {
        let input = " | ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Pipe);
    }

    /*
        Test lexer with assign
        - test_lexer_assign
        - test_lexer_assign_with_space
    */

    #[test]
    fn test_lexer_assign() {
        let input = "=";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Assign);
    }

    #[test]
    fn test_lexer_assign_with_space() {
        let input = " = ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Assign);
    }

    /*
        Test lexer with gt
        - test_lexer_gt
        - test_lexer_gt_with_space
    */

    #[test]
    fn test_lexer_gt() {
        let input = ">";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Gt);
    }

    #[test]
    fn test_lexer_gt_with_space() {
        let input = " > ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Gt);
    }

    /*
        Test lexer with lt
        - test_lexer_lt
        - test_lexer_lt_with_space
    */

    #[test]
    fn test_lexer_lt() {
        let input = "<";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Lt);
    }

    #[test]
    fn test_lexer_lt_with_space() {
        let input = " < ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Lt);
    }

    /*
        Test lexer with plus
        - test_lexer_plus
        - test_lexer_plus_with_space
    */

    // #[test]
    // fn test_lexer_plus() {
    //     let input = "+";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Plus);
    // }

    // #[test]
    // fn test_lexer_plus_with_space() {
    //     let input = " + ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Plus);
    // }

    /*
        Test lexer with minus
        - test_lexer_minus
        - test_lexer_minus_with_space
    */

    // #[test]
    // fn test_lexer_minus() {
    //     let input = "-";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Minus);
    // }

    // #[test]
    // fn test_lexer_minus_with_space() {
    //     let input = " - ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Minus);
    // }

    /*
        Test lexer with star
        - test_lexer_star
        - test_lexer_star_with_space
    */

    // #[test]
    // fn test_lexer_star() {
    //     let input = "*";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Star);
    // }

    // #[test]
    // fn test_lexer_star_with_space() {
    //     let input = " * ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Star);
    // }

    /*
        Test lexer with slash
        - test_lexer_slash
        - test_lexer_slash_with_space
    */

    // #[test]
    // fn test_lexer_slash() {
    //     let input = "/";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Slash);
    // }

    // #[test]
    // fn test_lexer_slash_with_space() {
    //     let input = " / ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::Slash);
    // }

    /*
        Test lexer with ampersand
        - test_lexer_ampersand
        - test_lexer_ampersand_with_space
    */

    #[test]
    fn test_lexer_ampersand() {
        let input = "&";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Ampersand);
    }

    #[test]
    fn test_lexer_ampersand_with_space() {
        let input = " & ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Ampersand);
    }

    /*
        Test lexer with left paren
        - test_lexer_left_paren
        - test_lexer_left_paren_with_space
    */

    // #[test]
    // fn test_lexer_left_paren() {
    //     let input = "(";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::LeftParen);
    // }

    // #[test]
    // fn test_lexer_left_paren_with_space() {
    //     let input = " ( ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::LeftParen);
    // }

    /*
        Test lexer with right paren
        - test_lexer_right_paren
        - test_lexer_right_paren_with_space
    */

    // #[test]
    // fn test_lexer_right_paren() {
    //     let input = ")";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::RightParen);
    // }

    // #[test]
    // fn test_lexer_right_paren_with_space() {
    //     let input = " ) ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::RightParen);
    // }

    /*
        Test lexer with dollar
        - test_lexer_dollar
        - test_lexer_dollar_with_space
    */

    #[test]
    fn test_lexer_dollar() {
        let input = "$";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Dollar);
    }

    #[test]
    fn test_lexer_dollar_with_space() {
        let input = " $ ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Dollar);
    }

    /*
        Test lexer with fd
        - test_lexer_fd
        - test_lexer_fd_with_space
    */

    #[test]
    fn test_lexer_fd() {
        let input = "@123";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::FD(123));
    }

    #[test]
    fn test_lexer_fd_with_space() {
        let input = " @123 ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::FD(123));
    }

    /*
        Test lexer with usize
        - test_lexer_usize
        - test_lexer_usize_with_space
    */

    #[test]
    fn test_lexer_usize() {
        let input = "123";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::USize(123));
    }

    #[test]
    fn test_lexer_usize_with_space() {
        let input = " 123 ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::USize(123));
    }

    /*
        Test lexer with isize
        - test_lexer_isize
        - test_lexer_isize_with_space
    */
    // #[test]
    // fn test_lexer_isize() {
    //     let input = "-123";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::ISize(-123));
    // }

    // #[test]
    // fn test_lexer_isize_with_space() {
    //     let input = " -123 ";

    //     let mut lexer = Lexer::new(input);

    //     assert_eq!(lexer.next().unwrap(), Token::ISize(-123));
    // }

    /*
        Test lexer with eof
        - test_lexer_eof
        - test_lexer_eof_with_space
        - test_lexer_some(eof)_none
    */

    #[test]
    fn test_lexer_eof() {
        let input = "";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_eof_with_space() {
        let input = " ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_some_eof_none() {
        let input = " ";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::EOF);

        assert_eq!(lexer.next(), None);
    }

    /*

        Test lexer with string
        - test_lexer_string
        - test_lexer_string_with_space
        - test_lexer_string_double_quote
    */

    #[test]
    fn test_lexer_string() {
        let input = "abcd";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::String("abcd".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_string_with_space() {
        let input = "abcd efgh";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::String("abcd".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::String("efgh".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_string_double_quote() {
        let input = r#"
            "hello" "world" "hello world"
        "#;

        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::String("hello world".to_string()),
            Token::EOF,
        ];

        for tkn in expected {
            assert_eq!(lexer.next(), Some(tkn));
        }
    }

    /*
        Test lexer with ident
        - test_lexer_ident
        - test_lexer_ident_with_space
        - test_lexer_ident_double_quote
    */

    #[test]
    fn test_lexer_ident() {
        let input = "$hello";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Ident("hello".to_string()));
    }

    #[test]
    fn test_lexer_ident_with_space() {
        let input = "$hello world";

        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap(), Token::Ident("hello".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::String("world".to_string()));
    }

    #[test]
    fn test_lexer_ident_double_quote() {
        let input = "$\"hello world\"";

        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next().unwrap(),
            Token::Ident("hello world".to_string())
        );
    }

    /*
        Test lexer with mixed
        - test_lexer_mixed
    */

    #[test]
    fn test_lexer_mixed() {
        //$hello "world" 123 456 -10 @789 () 0 ) ( + - * /
        //$hello "world" 123 456 @789
        let input = r#"
            $hello "world" 123 456 @789
        "#;

        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Ident("hello".to_string()),
            Token::String("world".to_string()),
            Token::USize(123),
            Token::USize(456),
            // Token::ISize(-10),
            Token::FD(789),
            // Token::LeftParen,
            // Token::RightParen,
            // Token::USize(0),
            // Token::RightParen,
            // Token::LeftParen,
            // Token::Plus,
            // Token::Minus,
            // Token::Star,
            // Token::Slash,
            Token::EOF,
        ];

        for tkn in expected {
            assert_eq!(lexer.next(), Some(tkn));
        }
    }
}
