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

fn parse_redirect(tokens: &[Token]) -> Result<flat_ast::Redirect> {
    let len = tokens.len();

    if !(len == 2 || len == 3) {
        Err(Error::DUMMY)?;
    }

    let (left, mut op) = if let Some(token) = &tokens.get(0) {
        let (left, op): (flat_ast::Expr, Option<flat_ast::RecirectOperator>) = match token {
            Token::FD(fd) => (flat_ast::Expr::FD(*fd), None),
            Token::Gt => (flat_ast::Expr::FD(1), Some(flat_ast::RecirectOperator::Gt)),
            Token::Lt => (flat_ast::Expr::FD(0), Some(flat_ast::RecirectOperator::Lt)),
            _ => Err(Error::DUMMY)?,
        };

        (left, op)
    } else {
        Err(Error::DUMMY)?
    };

    let right = if op.is_none() {
        if let Some(token) = &tokens.get(1) {
            match token {
                Token::Gt => op = Some(flat_ast::RecirectOperator::Gt),
                Token::Lt => op = Some(flat_ast::RecirectOperator::Lt),
                _ => Err(Error::DUMMY)?,
            };
        }

        if let Some(token) = &tokens.get(2) {
            match token {
                Token::String(string) => flat_ast::Expr::String(string.to_owned()),
                Token::Ident(ident) => flat_ast::Expr::Ident(ident.to_owned()),
                Token::USize(num) => flat_ast::Expr::USize(*num),
                Token::FD(fd) => flat_ast::Expr::FD(*fd),
                _ => Err(Error::DUMMY)?,
            }
        } else {
            Err(Error::DUMMY)?
        }
    } else {
        if let Some(token) = tokens.get(1) {
            match token {
                Token::String(string) => flat_ast::Expr::String(string.to_owned()),
                Token::Ident(ident) => flat_ast::Expr::Ident(ident.to_owned()),
                Token::USize(num) => flat_ast::Expr::USize(*num),
                Token::FD(fd) => flat_ast::Expr::FD(*fd),
                _ => Err(Error::DUMMY)?,
            }
        } else {
            Err(Error::DUMMY)?
        }
    };

    Ok(flat_ast::Redirect {
        left,
        right,
        operator: op.unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assign() {
        let tokens = [
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::String("b".to_string()),
        ];

        let assign = parse_assign(&tokens).unwrap();

        assert_eq!(assign.ident, flat_ast::Expr::Ident("a".to_string()));
        assert_eq!(assign.expr, flat_ast::Expr::String("b".to_string()));
    }

    #[test]
    fn test_parse_command() {
        let mut tokens = [
            Token::Ident("a".to_string()),
            Token::String("b".to_string()),
            Token::USize(1),
        ];

        let command = parse_command(&mut tokens).unwrap();

        assert_eq!(command.expr, flat_ast::Expr::Ident("a".to_string()));
        assert_eq!(
            command.args,
            vec![
                flat_ast::Expr::String("b".to_string()),
                flat_ast::Expr::USize(1),
            ]
        );
    }

    #[test]
    fn test_parse_redirect_pattern1() {
        // @1 > filename
        let tokens = [
            Token::FD(1),
            Token::Gt,
            Token::String("filename".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::String("filename".to_string()));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_pattern2() {
        // > filename
        let tokens = [Token::Gt, Token::String("filename".to_string())];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::String("filename".to_string()));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_pattern3() {
        // @1 > @2
        let tokens = [Token::FD(1), Token::Gt, Token::FD(2)];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::FD(2));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_pattern4() {
        // @1 > filename
        let tokens = [
            Token::FD(1),
            Token::Gt,
            Token::String("filename".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::String("filename".to_string()));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_pattern5() {
        // @1 < filename
        let tokens = [
            Token::FD(1),
            Token::Lt,
            Token::String("filename".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::String("filename".to_string()));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Lt);
    }

    #[test]
    fn test_parse_redirect_pattern6() {
        // < filename
        let tokens = [Token::Lt, Token::String("filename".to_string())];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(0));
        assert_eq!(redirect.right, flat_ast::Expr::String("filename".to_string()));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Lt);
    }

    #[test]
    fn test_parse_redirect_pattern7() {
        // @1 < @2
        let tokens = [Token::FD(1), Token::Lt, Token::FD(2)];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::FD(2));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Lt);
    }

    #[test]
    fn test_parse_redirect_pattern8() {
        // @1 < filename
        let tokens = [
            Token::FD(1),
            Token::Lt,
            Token::String("filename".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::String("filename".to_string()));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Lt);
    }
}
