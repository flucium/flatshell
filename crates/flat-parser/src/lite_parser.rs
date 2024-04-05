use flat_ast::{Assign, Command, Expr, FlatAst, Pipe, Redirect, RedirectOperator};
use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};

use super::{token::Token, utils};

/// Parse a string token.
pub fn parse_string(token: &Token) -> Result<Expr> {
    match token {
        Token::String(s) => Ok(Expr::String(s.to_string())),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected string")),
    }
}

/// Parse an ident token.
pub fn parse_ident(token: &Token) -> Result<Expr> {
    match token {
        Token::Ident(s) => Ok(Expr::Ident(s.to_string())),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected ident")),
    }
}

/// Parse a number token.
pub fn parse_number(token: &Token) -> Result<Expr> {
    match token {
        Token::USize(n) => Ok(Expr::USize(*n)),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected number")),
    }
}

/// Parse an fd token.
pub fn parse_fd(token: &Token) -> Result<Expr> {
    match token {
        Token::FD(n) => Ok(Expr::FD(*n)),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected fd")),
    }
}

/// Parse an expression token.
fn parse_expr(token: &Token, _exclude_tokens: Vec<Token>) -> Result<Expr> {
    for tkn in _exclude_tokens {
        if token == &tkn {
            Err(Error::new(ErrorKind::SyntaxError, "Expected expr"))?
        }
    }

    match token {
        Token::String(s) => Ok(Expr::String(s.to_string())),
        Token::Ident(s) => Ok(Expr::Ident(s.to_string())),
        Token::USize(n) => Ok(Expr::USize(*n)),
        Token::FD(n) => Ok(Expr::FD(*n)),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected expr"))?,
    }
}

/// Parse an assign token.
pub fn parse_assign(tokens: &[Token; 3]) -> Result<Assign> {
    let ident = match &tokens[0] {
        Token::Ident(s) => Expr::Ident(s.to_owned()),
        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected ident"))?,
    };

    if tokens[1] != Token::Assign {
        return Err(Error::new(ErrorKind::SyntaxError, "Expected assign"));
    }

    let expr = parse_expr(&tokens[2], vec![Token::Ident(String::default())])?;

    Ok(Assign { ident, expr })
}

/// Parse an abbreviated redirect.
fn parse_abbreviated_redirect(tokens: &[Token; 2]) -> Result<Redirect> {
    let op = match tokens[0] {
        Token::Gt => (Expr::FD(1), RedirectOperator::Gt),
        Token::Lt => (Expr::FD(0), RedirectOperator::Lt),
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected redirect operator",
        ))?,
    };

    let expr = parse_expr(&tokens[1], Vec::default())?;

    Ok(Redirect {
        left: op.0,
        right: expr,
        operator: op.1,
    })
}

/// Parse a normal redirect.
fn parse_normal_redirect(tokens: &[Token; 3]) -> Result<Redirect> {
    let left = parse_fd(&tokens[0])?;

    let op = match tokens[1] {
        Token::Gt => RedirectOperator::Gt,
        Token::Lt => RedirectOperator::Lt,
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected redirect operator",
        ))?,
    };
    let right = parse_expr(&tokens[2], Vec::default())?;

    Ok(Redirect {
        left,
        right,
        operator: op,
    })
}

/// Parse a redirect.
pub fn parse_redirect(tokens: &[Token]) -> Result<Redirect> {
    match tokens.len() {
        2 => parse_abbreviated_redirect(tokens[0..2].try_into().unwrap()),

        3 => parse_normal_redirect(tokens[0..3].try_into().unwrap()),

        _ => Err(Error::new(ErrorKind::SyntaxError, "Expected redirect")),
    }
}

/// Parse a command expression.
fn parse_command_expr(token: &Token) -> Result<Expr> {
    parse_expr(token, vec![Token::FD(0)])
}

/// Parse command arguments and redirects.
fn parse_command_args_and_redirects(tokens: &[Token]) -> Result<(Vec<Expr>, Vec<Redirect>)> {
    let mut args = Vec::new();

    let mut redirects = Vec::new();

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
                let arg = parse_expr(&tokens[i], vec![Token::Gt, Token::Lt, Token::FD(0)])?;
                args.push(arg);
            }
        }
    }

    Ok((args, redirects))
}

/// Parse a command.
pub fn parse_command(tokens: &[Token]) -> Result<Command> {
    let expr = parse_command_expr(&tokens[0])?;

    let (tokens, background) = if tokens[1..].contains(&Token::Ampersand) {
        let (left, _) = utils::split(&Token::Ampersand, &tokens[1..]);
        (left, true)
    } else {
        (tokens[1..].to_vec(), false)
    };

    let (args, redirects) = parse_command_args_and_redirects(&tokens)?;

    Ok(Command {
        expr,
        args,
        redirects,
        background,
    })
}

/// Parse a pipe.
pub fn parse_pipe(tokens: &[Token]) -> Result<Pipe> {
    if tokens.len() == 0 {
        Err(Error::DUMMY)?
    }

    if tokens.len() == 1 {
        if tokens[0] == Token::Pipe {
            Err(Error::DUMMY)?
        }
    }

    let mut pipe = Pipe::new();

    let command_tokens = utils::recursion_split(&Token::Pipe, tokens);

    for command in command_tokens {
        let command = parse_command(&command)?;

        pipe.push_back(command);
    }

    Ok(pipe)
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn test_parse_string() {
        let token = Token::String("hello".to_string());

        let expr = parse_string(&token).unwrap();

        assert_eq!(expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_parse_ident() {
        let token = Token::Ident("hello".to_string());

        let expr = parse_ident(&token).unwrap();

        assert_eq!(expr, Expr::Ident("hello".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let token = Token::USize(100);

        let expr = parse_number(&token).unwrap();

        assert_eq!(expr, Expr::USize(100));
    }

    #[test]
    fn test_parse_fd() {
        let token = Token::FD(1);

        let expr = parse_fd(&token).unwrap();

        assert_eq!(expr, Expr::FD(1));
    }

    #[test]
    fn test_parse_assign() {
        let tokens = [
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::String("hello".to_string()),
        ];

        let assign = parse_assign(&tokens).unwrap();

        assert_eq!(assign.ident, Expr::Ident("a".to_string()));
        assert_eq!(assign.expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_parse_abbreviated_redirect() {
        let redirect1 =
            parse_abbreviated_redirect(&[Token::Gt, Token::String("hello".to_string())]).unwrap();

        let redirect2 =
            parse_abbreviated_redirect(&[Token::Lt, Token::String("hello".to_string())]).unwrap();

        assert_eq!(redirect1.left, Expr::FD(1));
        assert_eq!(redirect1.right, Expr::String("hello".to_string()));
        assert_eq!(redirect1.operator, RedirectOperator::Gt);

        assert_eq!(redirect2.left, Expr::FD(0));
        assert_eq!(redirect2.right, Expr::String("hello".to_string()));
        assert_eq!(redirect2.operator, RedirectOperator::Lt);
    }

    #[test]
    fn test_parse_normal_redirect() {
        let redirect1 =
            parse_normal_redirect(&[Token::FD(1), Token::Gt, Token::String("hello".to_string())])
                .unwrap();

        let redirect2 =
            parse_normal_redirect(&[Token::FD(0), Token::Lt, Token::String("hello".to_string())])
                .unwrap();

        assert_eq!(redirect1.left, Expr::FD(1));
        assert_eq!(redirect1.right, Expr::String("hello".to_string()));
        assert_eq!(redirect1.operator, RedirectOperator::Gt);

        assert_eq!(redirect2.left, Expr::FD(0));
        assert_eq!(redirect2.right, Expr::String("hello".to_string()));
        assert_eq!(redirect2.operator, RedirectOperator::Lt);
    }

    #[test]
    fn test_parse_redirect() {
        let redirect1 = parse_redirect(&[Token::Gt, Token::String("hello".to_string())]).unwrap();

        let redirect2 =
            parse_redirect(&[Token::FD(1), Token::Gt, Token::String("hello".to_string())]).unwrap();

        assert_eq!(redirect1.left, Expr::FD(1));
        assert_eq!(redirect1.right, Expr::String("hello".to_string()));
        assert_eq!(redirect1.operator, RedirectOperator::Gt);

        assert_eq!(redirect2.left, Expr::FD(1));
        assert_eq!(redirect2.right, Expr::String("hello".to_string()));
        assert_eq!(redirect2.operator, RedirectOperator::Gt);
    }

    #[test]
    fn test_parse_command_expr() {
        assert_eq!(
            parse_command_expr(&Token::String("hello".to_string())).unwrap(),
            Expr::String("hello".to_string())
        );
        assert_eq!(
            parse_command_expr(&Token::Ident("hello".to_string())).unwrap(),
            Expr::Ident("hello".to_string())
        );
        assert_eq!(
            parse_command_expr(&Token::USize(100)).unwrap(),
            Expr::USize(100)
        );
    }

    #[test]
    fn test_parse_command_expr_error() {
        assert!(parse_command_expr(&Token::FD(0)).is_err());
    }

    #[test]
    fn test_parse_command_args_and_redirects() {
        let tokens = [
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::Gt,
            Token::String("file".to_string()),
        ];

        let (args, redirects) = parse_command_args_and_redirects(&tokens).unwrap();

        assert_eq!(args.len(), 2);

        assert_eq!(redirects.len(), 1);

        assert_eq!(args[0], Expr::String("hello".to_string()));
        assert_eq!(args[1], Expr::String("world".to_string()));

        assert_eq!(redirects[0].left, Expr::FD(1));
        assert_eq!(redirects[0].right, Expr::String("file".to_string()));
        assert_eq!(redirects[0].operator, RedirectOperator::Gt);
    }

    #[test]
    fn test_parse_command() {
        let command1 = parse_command(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
        ]);

        let command2 = parse_command(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::Gt,
            Token::String("file".to_string()),
            Token::FD(2),
            Token::Gt,
            Token::String("file2".to_string()),
        ]);

        assert_eq!(
            command1.unwrap(),
            Command {
                expr: Expr::String("echo".to_string()),
                args: vec![
                    Expr::String("hello".to_string()),
                    Expr::String("world".to_string())
                ],
                redirects: Vec::default(),
                background: false
            }
        );

        assert_eq!(
            command2.unwrap(),
            Command {
                expr: Expr::String("echo".to_string()),
                args: vec![
                    Expr::String("hello".to_string()),
                    Expr::String("world".to_string())
                ],
                redirects: vec![
                    Redirect {
                        left: Expr::FD(1),
                        right: Expr::String("file".to_string()),
                        operator: RedirectOperator::Gt
                    },
                    Redirect {
                        left: Expr::FD(2),
                        right: Expr::String("file2".to_string()),
                        operator: RedirectOperator::Gt
                    }
                ],
                background: false
            }
        );
    }

    #[test]
    fn test_parse_command_background() {
        let command1 = parse_command(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::Ampersand,
        ]);

        let command2 = parse_command(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::Gt,
            Token::String("file".to_string()),
            Token::FD(2),
            Token::Gt,
            Token::String("file2".to_string()),
            Token::Ampersand,
        ]);

        assert_eq!(
            command1.unwrap(),
            Command {
                expr: Expr::String("echo".to_string()),
                args: vec![
                    Expr::String("hello".to_string()),
                    Expr::String("world".to_string())
                ],
                redirects: Vec::default(),
                background: true
            }
        );

        assert_eq!(
            command2.unwrap(),
            Command {
                expr: Expr::String("echo".to_string()),
                args: vec![
                    Expr::String("hello".to_string()),
                    Expr::String("world".to_string())
                ],
                redirects: vec![
                    Redirect {
                        left: Expr::FD(1),
                        right: Expr::String("file".to_string()),
                        operator: RedirectOperator::Gt
                    },
                    Redirect {
                        left: Expr::FD(2),
                        right: Expr::String("file2".to_string()),
                        operator: RedirectOperator::Gt
                    }
                ],
                background: true
            }
        );
    }

    #[test]
    fn test_parse_pipe() {
        let pipe1 = parse_pipe(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::Pipe,
            Token::String("cat".to_string()),
            Token::String("-b".to_string()),
        ]);

        let pipe2 = parse_pipe(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::Pipe,
            Token::String("cat".to_string()),
            Token::String("-b".to_string()),
            Token::Pipe,
            Token::String("rev".to_string()),
        ]);

        let pipe3 = parse_pipe(&[
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::Pipe,
        ]);

        assert_eq!(
            pipe1.unwrap(),
            Pipe::from(
                [
                    Command {
                        expr: Expr::String("echo".to_string()),
                        args: vec![Expr::String("hello".to_string())],
                        redirects: Vec::default(),
                        background: false
                    },
                    Command {
                        expr: Expr::String("cat".to_string()),
                        args: vec![Expr::String("-b".to_string())],
                        redirects: Vec::default(),
                        background: false
                    }
                ]
                .iter()
                .cloned()
                .collect::<VecDeque<Command>>()
            )
        );

        assert_eq!(
            pipe2.unwrap(),
            Pipe::from(
                [
                    Command {
                        expr: Expr::String("echo".to_string()),
                        args: vec![Expr::String("hello".to_string())],
                        redirects: Vec::default(),
                        background: false
                    },
                    Command {
                        expr: Expr::String("cat".to_string()),
                        args: vec![Expr::String("-b".to_string())],
                        redirects: Vec::default(),
                        background: false
                    },
                    Command {
                        expr: Expr::String("rev".to_string()),
                        args: Vec::default(),
                        redirects: Vec::default(),
                        background: false
                    }
                ]
                .iter()
                .cloned()
                .collect::<VecDeque<Command>>()
            )
        );

        assert_eq!(
            pipe3.unwrap(),
            Pipe::from(
                [Command {
                    expr: Expr::String("echo".to_string()),
                    args: vec![Expr::String("hello".to_string())],
                    redirects: Vec::default(),
                    background: false
                }]
                .iter()
                .cloned()
                .collect::<VecDeque<Command>>()
            )
        );
    }

    #[test]
    fn test_parse_pipe_error() {
        assert!(parse_pipe(&[]).is_err());
        assert!(parse_pipe(&[Token::Pipe]).is_err());
    }
}
