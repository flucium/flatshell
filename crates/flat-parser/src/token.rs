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

    Semicolon, // ;
    Dollar,    // $
    Ampersand, // &

    String(String), // hello
    Ident(String),  // $a , &b
    USize(usize),   // 0 ~ 9
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
            Token::Semicolon => write!(tkn, ";"),
            Token::Dollar => write!(tkn, "$"),
            Token::Ampersand => write!(tkn, "&"),
            Token::String(v) => write!(tkn, "{v}"),
            Token::Ident(v) => write!(tkn, "{v}"),
            Token::USize(v) => write!(tkn, "{v}"),
            Token::FD(v) => write!(tkn, "{v}"),
        }
    }
}
