use flat_ast;
use flat_common::error::Error;
use flat_common::result::Result;

use crate::token::{self, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    ast: flat_ast::FlatAst,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
            ast: flat_ast::FlatAst::new(),
        }
    }

    pub fn parse(&mut self) {
        let start = self.position;

        while let Some(tkn) = self.tokens.get(self.position) {
            self.position += 1;

            if tkn == &Token::EOF {
                break;
            }

            
        }
    }
}

fn parse_assign(tokens: &[Token; 3]) -> Result<flat_ast::Assign> {
    let ident = match &tokens[0] {
        Token::Ident(string) => flat_ast::Expr::Ident(string.to_string()),
        _ => Err(Error::DUMMY)?,
    };

    if tokens[1] != Token::Assign {
        Err(Error::DUMMY)?;
    }

    let expr = match &tokens[2] {
        Token::String(string) => flat_ast::Expr::String(string.to_string()),
        Token::USize(num) => flat_ast::Expr::USize(*num),
        _ => Err(Error::DUMMY)?,
    };

    Ok(flat_ast::Assign { ident, expr })
}

fn parse_command(tokens: &mut [Token]) -> Result<flat_ast::Command> {
    if tokens.len() == 0 {
        Err(Error::DUMMY)?;
    }

    let expr = match &tokens[0] {
        Token::String(string) => flat_ast::Expr::String(string.to_string()),
        Token::Ident(string) => flat_ast::Expr::Ident(string.to_string()),
        Token::USize(num) => flat_ast::Expr::USize(*num),
        _ => Err(Error::DUMMY)?,
    };

    let mut args = Vec::new();

    for tkn in tokens[1..].iter() {
        match tkn {
            Token::String(string) => args.push(flat_ast::Expr::String(string.to_string())),
            Token::Ident(string) => args.push(flat_ast::Expr::Ident(string.to_string())),
            Token::USize(num) => args.push(flat_ast::Expr::USize(*num)),
            _ => Err(Error::DUMMY)?,
        }
    }

    Ok(flat_ast::Command { expr, args })
}
