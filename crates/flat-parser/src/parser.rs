/*
    Todo:
    1 Refactor the parse assign.
    2 Refactor the parse redirect.
    3 Refactor the parse command.
    4 Refactor the parse pipe.
    5 Refactor the semicolon split.
    6 Implements background execution commands (in combination with nohup) into commands and command parsing.
    7 Implements Close FD.
    8 Refactor the parse.
    10 Refactor the unit tests.
    
*/

use flat_ast;
use flat_common::error::{Error, ErrorKind};
use flat_common::result::Result;

use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    // ast: flat_ast::FlatAst,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        // let ast = flat_ast::FlatAst::Semicolon(Vec::new());
        Parser { tokens }
    }
    /*
        This is a draft!!!
        This is a draft!!!
        This is a draft!!!
        This is a draft!!!
        This is a draft!!!
        This is a draft!!!
    */
    pub fn parse(&mut self) -> Result<flat_ast::FlatAst> {
        if self.tokens.last() == Some(&Token::EOF) {
            self.tokens.pop().unwrap();
        } else {
            Err(Error::DUMMY)?
        }

        let mut semicolon_node = Vec::new();

        let entries = split_semicolon(&mut self.tokens).unwrap();

        for mut tokens in entries {
            if tokens.contains(&Token::Pipe) {
                semicolon_node.push(flat_ast::FlatAst::Pipe(parse_pipe(&mut tokens)?));
            } else if tokens.len() == 3 && tokens.contains(&Token::Assign) {
                semicolon_node.push(flat_ast::FlatAst::Statement(flat_ast::Statement::Assign(
                    parse_assign(&TryInto::<[Token; 3]>::try_into(tokens).unwrap())?,
                )));
            } else {
                semicolon_node.push(flat_ast::FlatAst::Statement(flat_ast::Statement::Command(
                    parse_command(&mut tokens)?,
                )));
            }
        }

        Ok(flat_ast::FlatAst::Semicolon(semicolon_node))
    }
}

fn split_semicolon(tokens: &mut [Token]) -> Result<Vec<Vec<Token>>> {
    let mut commands = Vec::new();

    let mut command_tokens = Vec::new();

    for token in tokens.iter() {
        match token {
            Token::Semicolon => {
                commands.push(command_tokens);

                command_tokens = Vec::new();
            }
            _ => {
                command_tokens.push(token.to_owned());
            }
        }
    }

    if !command_tokens.is_empty() {
        commands.push(command_tokens);
    }

    Ok(commands)
}

fn parse_pipe(tokens: &mut [Token]) -> Result<flat_ast::Pipe> {
    let mut commands = Vec::new();

    let mut command_tokens = Vec::new();

    for token in tokens.iter() {
        match token {
            Token::Pipe => {
                let command = parse_command(&mut command_tokens)?;

                commands.push(command);

                command_tokens = Vec::new();
            }
            _ => {
                command_tokens.push(token.to_owned());
            }
        }
    }

    if !command_tokens.is_empty() {
        let command = parse_command(&mut command_tokens)?;

        commands.push(command);
    }

    Ok(flat_ast::Pipe { commands })
}

fn parse_assign(tokens: &[Token; 3]) -> Result<flat_ast::Assign> {
    let ident = parse_ident(&tokens[0])?;

    if tokens[1] != Token::Assign {
        Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected an assignment operator",
        ))?;
    }

    let expr = parse_string(&tokens[2]).or(parse_usize(&tokens[2]))?;

    Ok(flat_ast::Assign { ident, expr })
}

fn parse_command(tokens: &mut [Token]) -> Result<flat_ast::Command> {
    if tokens.len() == 0 {
        Err(Error::DUMMY)?;
    }

    let expr = parse_string(&tokens[0])
        .or(parse_ident(&tokens[0]))
        .or(parse_usize(&tokens[0]))?;

    let mut args = Vec::new();

    let mut redirects = Vec::new();

    let tokens = tokens[1..].to_vec();

    let mut skip_count = 0;

    for i in 0..tokens.len() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match tokens[i] {
            Token::Gt | Token::Lt => {
                if i + 1 < tokens.len() {
                    let redirect = parse_redirect(&tokens[i..i + 2])?;

                    redirects.push(redirect);

                    skip_count = 1;
                } else {
                    Err(Error::DUMMY)?;
                }
            }

            Token::FD(_) => {
                if i + 2 < tokens.len() {
                    let redirect = parse_redirect(&tokens[i..i + 3])?;

                    redirects.push(redirect);

                    skip_count = 2;
                } else {
                    Err(Error::DUMMY)?;
                }
            }
            _ => {
                let arg = parse_string(&tokens[i])
                    .or(parse_ident(&tokens[i]))
                    .or(parse_usize(&tokens[i]))?;

                args.push(arg);
            }
        }
    }

    Ok(flat_ast::Command {
        expr,
        args,
        redirects,
    })
}

fn parse_redirect(tokens: &[Token]) -> Result<flat_ast::Redirect> {
    let len = tokens.len();

    if !(len == 2 || len == 3) {
        Err(Error::DUMMY)?;
    }

    let (left, mut op): (flat_ast::Expr, Option<flat_ast::RecirectOperator>) =
        if let Some(token) = tokens.get(0) {
            if let Ok(left) = parse_fd(token) {
                (left, None)
            } else {
                match token {
                    Token::Gt => (flat_ast::Expr::FD(1), Some(flat_ast::RecirectOperator::Gt)),

                    Token::Lt => (flat_ast::Expr::FD(0), Some(flat_ast::RecirectOperator::Lt)),
                    _ => Err(Error::DUMMY)?,
                }
            }
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
            parse_string(token)
                .or(parse_ident(token))
                .or(parse_usize(token))
                .or(parse_fd(token))?
        } else {
            Err(Error::DUMMY)?
        }
    } else {
        if let Some(token) = tokens.get(1) {
            parse_string(token)
                .or(parse_ident(token))
                .or(parse_usize(token))
                .or(parse_fd(token))?
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

// fn parse_close_fd(token: &Token) -> Result<flat_ast::Expr> {
//     match token {
//         Token::FD(fd) => {
//             if *fd < 0 {
//                 Ok(flat_ast::Expr::FD(*fd))
//             } else {
//                 Err(Error::DUMMY)?
//             }
//         }
//         _ => Err(Error::DUMMY)?,
//     }
// }

/// Parse a file descriptor literal
fn parse_fd(token: &Token) -> Result<flat_ast::Expr> {
    match token {
        Token::FD(fd) => Ok(flat_ast::Expr::FD(*fd)),
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected a file descriptor literal",
        ))?,
    }
}

/// Parse a usize literal
fn parse_usize(token: &Token) -> Result<flat_ast::Expr> {
    match token {
        Token::USize(num) => Ok(flat_ast::Expr::USize(*num)),
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected a usize literal",
        ))?,
    }
}

/// Parse an identifier
fn parse_ident(token: &Token) -> Result<flat_ast::Expr> {
    match token {
        Token::Ident(string) => Ok(flat_ast::Expr::Ident(string.to_string())),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected a identifier"))?,
    }
}

/// Parse a string literal
fn parse_string(token: &Token) -> Result<flat_ast::Expr> {
    match token {
        Token::String(string) => Ok(flat_ast::Expr::String(string.to_string())),
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected a string literal",
        ))?,
    }
}