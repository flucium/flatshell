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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_with_redirect() {
        let mut tokens = vec![
            Token::Ident("ls".to_string()),
            Token::String("-a".to_string()),
            Token::Gt,
            Token::String("file".to_string()),
            Token::String("~".to_string()),
            Token::FD(2),
            Token::Gt,
            Token::String("errfile".to_string()),
        ];

        assert_eq!(parse_command(&mut tokens).is_ok(), true);
    }

    /*
        Test parse_string
            - test_parse_string
            - test_parse_string_with_space
            - test_parse_string_empty
    */

    #[test]
    fn test_parse_string() {
        let token = Token::String("string".to_string());

        let expr = parse_string(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::String("string".to_string()));
    }

    #[test]
    fn test_parse_string_with_space() {
        let token = Token::String("string with space".to_string());

        let expr = parse_string(&token).unwrap();

        assert_eq!(
            expr,
            flat_ast::Expr::String("string with space".to_string())
        );
    }

    #[test]
    fn test_parse_string_empty() {
        let token = Token::String("".to_string());

        let expr = parse_string(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::String("".to_string()));

        assert_eq!(expr, flat_ast::Expr::String(String::new()));
    }

    /*
        Test parse_ident
            - test_parse_ident
            - test_parse_ident_with_space
            - test_parse_ident_empty
    */

    #[test]
    fn test_parse_ident() {
        let token = Token::Ident("ident".to_string());

        let expr = parse_ident(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::Ident("ident".to_string()));
    }

    #[test]
    fn test_parse_ident_with_space() {
        let token = Token::Ident("ident with space".to_string());

        let expr = parse_ident(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::Ident("ident with space".to_string()));
    }

    #[test]
    fn test_parse_ident_empty() {
        let token = Token::Ident("".to_string());

        let expr = parse_ident(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::Ident("".to_string()));

        assert_eq!(expr, flat_ast::Expr::Ident(String::new()));
    }

    /*
        Test parse_usize
            - test_parse_usize
            - test_parse_usize_max
            - test_parse_usize_min
    */

    #[test]
    fn test_parse_usize() {
        let token = Token::USize(1);

        let expr = parse_usize(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::USize(1));
    }

    #[test]
    fn test_parse_usize_max() {
        let token = Token::USize(usize::MAX);

        let expr = parse_usize(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::USize(usize::MAX));
    }

    #[test]
    fn test_parse_usize_min() {
        let token = Token::USize(usize::MIN);

        let expr = parse_usize(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::USize(usize::MIN));
    }

    /*
        Test parse_fd
            - test_parse_fd
            - test_parse_fd_close
    */

    #[test]
    fn test_parse_fd() {
        let token = Token::FD(1);

        let expr = parse_fd(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::FD(1));
    }

    // #[test]
    // fn test_parse_fd_close() {
    //     let token = Token::FD(-1);

    //     let expr = parse_close_fd(&token).unwrap();

    //     assert_eq!(expr, flat_ast::Expr::FD(-1));
    // }

    /*
        Test parse assign
            - test_parse_assign
            - test_parse_assign_string_empty
    */
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
    fn test_parse_assign_string_empty() {
        let tokens = [
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::String("".to_string()),
        ];

        let assign = parse_assign(&tokens).unwrap();

        assert_eq!(assign.ident, flat_ast::Expr::Ident("a".to_string()));
        assert_eq!(assign.expr, flat_ast::Expr::String("".to_string()));
    }

    /*
        Test parse command
            - test_parse_command
            - test_parse_command_and_args
    */

    #[test]
    fn test_parse_command() {
        let mut tokens = [Token::Ident("command".to_string())];

        let command = parse_command(&mut tokens).unwrap();

        assert_eq!(command.expr, flat_ast::Expr::Ident("command".to_string()));
        assert_eq!(command.args.len(), 0);
    }

    #[test]
    fn test_parse_command_and_args() {
        let mut tokens = [
            Token::Ident("command".to_string()),
            Token::String("arg1".to_string()),
            Token::String("arg2".to_string()),
            Token::USize(1),
        ];

        let command = parse_command(&mut tokens).unwrap();

        // assert command
        assert_eq!(command.expr, flat_ast::Expr::Ident("command".to_string()));

        // assert args
        assert_eq!(command.args[0], flat_ast::Expr::String("arg1".to_string()));
        assert_eq!(command.args[1], flat_ast::Expr::String("arg2".to_string()));
        assert_eq!(command.args[2], flat_ast::Expr::USize(1));

        // assert args length
        assert_eq!(command.args.len(), 3);
    }

    /*
        Test parse redirect
            - test_parse_redirect
            - test_parse_redirect_left_less_to_right
            - test_parse_redirect_left_and_right_are_fd
            - test_parse_redirect_left_fd_to_right_string
    */

    #[test]
    fn test_parse_redirect() {
        //
        // @1 > filename
        //
        let tokens = [
            Token::FD(1),
            Token::Gt,
            Token::String("filename".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));

        assert_eq!(
            redirect.right,
            flat_ast::Expr::String("filename".to_string())
        );

        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_left_less_to_right() {
        //
        // > filename
        //
        let tokens = [Token::Gt, Token::String("filename".to_string())];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));

        assert_eq!(
            redirect.right,
            flat_ast::Expr::String("filename".to_string())
        );

        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_left_and_right_are_fd() {
        //
        // @1 > @2
        //

        let tokens = [Token::FD(1), Token::Gt, Token::FD(2)];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));
        assert_eq!(redirect.right, flat_ast::Expr::FD(2));
        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_redirect_left_fd_to_right_string() {
        //
        // @1 > filename
        //

        let tokens = [
            Token::FD(1),
            Token::Gt,
            Token::String("filename".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.left, flat_ast::Expr::FD(1));

        assert_eq!(
            redirect.right,
            flat_ast::Expr::String("filename".to_string())
        );

        assert_eq!(redirect.operator, flat_ast::RecirectOperator::Gt);
    }

    #[test]
    fn test_parse_pipe() {
        //
        // ls | cat -b
        //

        let mut tokens = vec![
            Token::Ident("ls".to_string()),
            Token::Pipe,
            Token::Ident("cat".to_string()),
            Token::String("-b".to_string()),
        ];

        let pipe = parse_pipe(&mut tokens).unwrap();

        assert_eq!(pipe.commands.len(), 2);

        assert_eq!(
            pipe.commands[0].expr,
            flat_ast::Expr::Ident("ls".to_string())
        );

        assert_eq!(
            pipe.commands[1].expr,
            flat_ast::Expr::Ident("cat".to_string())
        );

        assert_eq!(
            pipe.commands[1].args[0],
            flat_ast::Expr::String("-b".to_string())
        );
    }
}
