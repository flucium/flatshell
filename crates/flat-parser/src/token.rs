use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    EOF,
    Pipe, // |
    Assign, // =
    Gt, // >
    Lt, // <
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    Semicolon, // ;
    Dollar,    // $
    Ampersand, // &
    LeftParen, // (
    RightParen, // )
    String(String), // hello
    Ident(String),  // $a , &b
    USize(usize),   // 0 ~ 9
    ISize(isize),   // 0 ~ 9 with negative
    FD(i32),        // 0 ~ 9 with negative
}

impl Display for Token {
    fn fmt(&self, tkn: &mut Formatter) -> Result {
        match self {
            Token::EOF => write!(tkn, "EOF"),
            Token::Pipe => write!(tkn, "|"),
            Token::Assign => write!(tkn, "="),
            Token::Gt => write!(tkn, ">"),
            Token::Lt => write!(tkn, "<"),
            Token::Plus => write!(tkn, "+"),
            Token::Minus => write!(tkn, "-"),
            Token::Star => write!(tkn, "*"),
            Token::Slash => write!(tkn, "/"),
            Token::Semicolon => write!(tkn, ";"),
            Token::Dollar => write!(tkn, "$"),
            Token::Ampersand => write!(tkn, "&"),
            Token::LeftParen => write!(tkn, "("),
            Token::RightParen => write!(tkn, ")"),
            Token::String(v) => write!(tkn, "{v}"),
            Token::Ident(v) => write!(tkn, "{v}"),
            Token::USize(v) => write!(tkn, "{v}"),
            Token::ISize(v) => write!(tkn, "{v}"),
            Token::FD(v) => write!(tkn, "{v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_display() {
        assert_eq!(Token::EOF.to_string(), "EOF");
        assert_eq!(Token::Pipe.to_string(), "|");
        assert_eq!(Token::Assign.to_string(), "=");
        assert_eq!(Token::Gt.to_string(), ">");
        assert_eq!(Token::Lt.to_string(), "<");
        assert_eq!(Token::Plus.to_string(), "+");
        assert_eq!(Token::Minus.to_string(), "-");
        assert_eq!(Token::Star.to_string(), "*");
        assert_eq!(Token::Slash.to_string(), "/");
        assert_eq!(Token::Semicolon.to_string(), ";");
        assert_eq!(Token::Dollar.to_string(), "$");
        assert_eq!(Token::Ampersand.to_string(), "&");
        assert_eq!(Token::LeftParen.to_string(), "(");
        assert_eq!(Token::RightParen.to_string(), ")");
        assert_eq!(Token::String("hello".to_string()).to_string(), "hello");
        assert_eq!(Token::Ident("a".to_string()).to_string(), "a");
        assert_eq!(Token::USize(1).to_string(), "1");
        assert_eq!(Token::ISize(1).to_string(), "1");
        assert_eq!(Token::FD(1).to_string(), "1");
    }
}