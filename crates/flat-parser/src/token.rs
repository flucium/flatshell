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
    // Plus, // +
    // Minus, // -
    // Star, // *
    // Slash, // /
    Semicolon, // ;
    Dollar,    // $
    Ampersand, // &
    // LeftParen, // (
    // RightParen, // )
    String(String), // hello
    Ident(String),  // $a , &b
    USize(usize),   // 0 ~ 9
    // ISize(isize),   // 0 ~ 9 with negative
    FD(i32),        // 0 ~ 9 with negative
}

impl Token{
    pub fn len(&self)->usize{
        match self {
            Token::EOF => 0,
            Token::Pipe => 1,
            Token::Assign => 1,
            Token::Gt => 1,
            Token::Lt => 1,
            // Token::Plus => 1,
            // Token::Minus => 1,
            // Token::Star => 1,
            // Token::Slash => 1,
            Token::Semicolon => 1,
            Token::Dollar => 1,
            Token::Ampersand => 1,
            // Token::LeftParen => 1,
            // Token::RightParen => 1,
            Token::String(v) => v.len(),
            Token::Ident(v) => v.len(),
            Token::USize(v) => v.to_string().len(),
            // Token::ISize(v) => v.to_string().len(),
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
            // Token::Plus => write!(tkn, "+"),
            // Token::Minus => write!(tkn, "-"),
            // Token::Star => write!(tkn, "*"),
            // Token::Slash => write!(tkn, "/"),
            Token::Semicolon => write!(tkn, ";"),
            Token::Dollar => write!(tkn, "$"),
            Token::Ampersand => write!(tkn, "&"),
            // Token::LeftParen => write!(tkn, "("),
            // Token::RightParen => write!(tkn, ")"),
            Token::String(v) => write!(tkn, "{v}"),
            Token::Ident(v) => write!(tkn, "{v}"),
            Token::USize(v) => write!(tkn, "{v}"),
            // Token::ISize(v) => write!(tkn, "{v}"),
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
        // assert_eq!(Token::Plus.len(), 1);
        // assert_eq!(Token::Minus.len(), 1);
        // assert_eq!(Token::Star.len(), 1);
        // assert_eq!(Token::Slash.len(), 1);
        assert_eq!(Token::Semicolon.len(), 1);
        assert_eq!(Token::Dollar.len(), 1);
        assert_eq!(Token::Ampersand.len(), 1);
        // assert_eq!(Token::LeftParen.len(), 1);
        // assert_eq!(Token::RightParen.len(), 1);
        assert_eq!(Token::String("hello".to_string()).len(), 5);
        assert_eq!(Token::Ident("a".to_string()).len(), 1);
        assert_eq!(Token::USize(1).len(), 1);
        // assert_eq!(Token::ISize(1).len(), 1);
        assert_eq!(Token::FD(1).len(), 1);
    }

    #[test]
    fn test_token_display() {
        assert_eq!(format!("{}", Token::EOF), "EOF");
        assert_eq!(format!("{}", Token::Pipe), "|");
        assert_eq!(format!("{}", Token::Assign), "=");
        assert_eq!(format!("{}", Token::Gt), ">");
        assert_eq!(format!("{}", Token::Lt), "<");
        // assert_eq!(format!("{}", Token::Plus), "+");
        // assert_eq!(format!("{}", Token::Minus), "-");
        // assert_eq!(format!("{}", Token::Star), "*");
        // assert_eq!(format!("{}", Token::Slash), "/");
        assert_eq!(format!("{}", Token::Semicolon), ";");
        assert_eq!(format!("{}", Token::Dollar), "$");
        assert_eq!(format!("{}", Token::Ampersand), "&");
        // assert_eq!(format!("{}", Token::LeftParen), "(");
        // assert_eq!(format!("{}", Token::RightParen), ")");
        assert_eq!(format!("{}", Token::String("hello".to_string())), "hello");
        assert_eq!(format!("{}", Token::Ident("a".to_string())), "a");
        assert_eq!(format!("{}", Token::USize(1)), "1");
        // assert_eq!(format!("{}", Token::ISize(1)), "1");
        assert_eq!(format!("{}", Token::FD(1)), "1");
    }
}