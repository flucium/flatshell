use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    EOF,
    Pipe,           // |
    Assign,         // =
    Gt,             // >
    Lt,             // <
    Semicolon,      // ;
    Dollar,         // $
    Ampersand,      // &
    String(String), // hello
    Ident(String),  // $a , &b
    Number(usize),  // 0 ~ 9
    FD(i32),        // 0 ~ 9 with negative
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::EOF => 0,
            Token::Pipe => 1,
            Token::Assign => 1,
            Token::Gt => 1,
            Token::Lt => 1,
            Token::Semicolon => 1,
            Token::Dollar => 1,
            Token::Ampersand => 1,
            Token::String(v) => v.len(),
            Token::Ident(v) => v.len(),
            Token::Number(v) => v.to_string().len(),
            Token::FD(v) => v.to_string().len(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, tkn: &mut Formatter) -> Result {
        match self {
            Token::EOF => write!(tkn, "EOF"),
            Token::Pipe => write!(tkn, "|"),
            Token::Assign => write!(tkn, "="),
            Token::Gt => write!(tkn, ">"),
            Token::Lt => write!(tkn, "<"),
            Token::Semicolon => write!(tkn, ";"),
            Token::Dollar => write!(tkn, "$"),
            Token::Ampersand => write!(tkn, "&"),
            Token::String(v) => write!(tkn, "{v}"),
            Token::Ident(v) => write!(tkn, "{v}"),
            Token::Number(v) => write!(tkn, "{v}"),
            Token::FD(v) => write!(tkn, "{v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_len() {
        assert_eq!(Token::EOF.len(), 0);
        assert_eq!(Token::Pipe.len(), 1);
        assert_eq!(Token::Assign.len(), 1);
        assert_eq!(Token::Gt.len(), 1);
        assert_eq!(Token::Lt.len(), 1);
        assert_eq!(Token::Semicolon.len(), 1);
        assert_eq!(Token::Dollar.len(), 1);
        assert_eq!(Token::Ampersand.len(), 1);
        assert_eq!(Token::String("hello".to_string()).len(), 5);
        assert_eq!(Token::Ident("a".to_string()).len(), 1);
        assert_eq!(Token::Number(1).len(), 1);
        assert_eq!(Token::FD(1).len(), 1);
    }

    #[test]
    fn test_token_display() {
        assert_eq!(format!("{}", Token::EOF), "EOF");
        assert_eq!(format!("{}", Token::Pipe), "|");
        assert_eq!(format!("{}", Token::Assign), "=");
        assert_eq!(format!("{}", Token::Gt), ">");
        assert_eq!(format!("{}", Token::Lt), "<");
        assert_eq!(format!("{}", Token::Semicolon), ";");
        assert_eq!(format!("{}", Token::Dollar), "$");
        assert_eq!(format!("{}", Token::Ampersand), "&");
        assert_eq!(format!("{}", Token::String("hello".to_string())), "hello");
        assert_eq!(format!("{}", Token::Ident("a".to_string())), "a");
        assert_eq!(format!("{}", Token::Number(1)), "1");
        assert_eq!(format!("{}", Token::FD(1)), "1");
    }
}
