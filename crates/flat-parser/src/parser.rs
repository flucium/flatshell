use flat_ast;
use flat_common::error::{Error, ErrorKind};
use flat_common::result::Result;

use super::Lexer;

use super::token::Token;
use super::utils;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    root: flat_ast::FlatAst,
}

impl Parser {
    /// Create a new parser
    pub fn new(lexer: Lexer) -> Self {
        let root = flat_ast::FlatAst::new();
        Self { lexer, root }
    }

    /// Parse the input
    ///
    /// This function will parse the input and return a flat_ast::FlatAst
    pub fn parse(&mut self) -> Result<flat_ast::FlatAst> {
        let mut tokens = self.lexer.tokenize();

        tokens.remove(tokens.len() - 1);

        let entries = utils::recursion_split(&Token::Semicolon, &tokens);

        for tokens in entries {

            if tokens.contains(&Token::Pipe) {
                let pipe = parse_pipe(&tokens)?;

                self.root.push(flat_ast::FlatAst::Pipe(pipe));
            } else {
                if tokens.len() == 3 {
                    let assign = parse_assign(&tokens.try_into().unwrap())?;

                    self.root
                        .push(flat_ast::FlatAst::Statement(flat_ast::Statement::Assign(
                            assign,
                        )));
                } else {
                    let command = parse_command(&tokens)?;

                    self.root
                        .push(flat_ast::FlatAst::Statement(flat_ast::Statement::Command(
                            command,
                        )));
                }
            }
        }

        Ok(self.root.clone())
    }
}

/// Parse a pipe
fn parse_pipe(tokens: &[Token]) -> Result<flat_ast::Pipe> {
    if tokens.len() == 0 {
        Err(Error::DUMMY)?;
    }

    if tokens.len() == 1 {
        if tokens[0] == Token::Pipe {
            Err(Error::new(ErrorKind::SyntaxError, "Expected a command"))?;
        }
    }

    let mut commands = Vec::new();

    let command_tokens = utils::recursion_split(&Token::Pipe, tokens);

    for command in command_tokens {
        let command = parse_command(&command)?;

        commands.push(command);
    }

    Ok(flat_ast::Pipe { commands })
}

/// Parse a command
fn parse_command(tokens: &[Token]) -> Result<flat_ast::Command> {
    if tokens.len() == 0 {
        Err(Error::DUMMY)?;
    }

    let expr = parse_command_expr(&tokens[0])?;

    let (tokens, background) = if tokens[1..].contains(&Token::Ampersand) {
        let (left, _) = utils::split(&Token::Ampersand, &tokens[1..]);
        (left, true)
    } else {
        (tokens[1..].to_vec(), false)
    };

    let (args, redirects) = parse_command_args_with_redirect(&tokens)?;

    Ok(flat_ast::Command {
        expr,
        args,
        redirects,
        background,
    })
}

/// Parse a command with arguments and redirects
///
/// This function is used to parse a command with arguments and redirects.
///
/// For example, if the input is `ls -a ~ > file.txt`, this function will parse the command `ls` with arguments `-a` and `~` and a redirect `> file.txt`.
fn parse_command_args_with_redirect(
    tokens: &[Token],
) -> Result<(Vec<flat_ast::Expr>, Vec<flat_ast::Redirect>)> {
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
                let arg = parse_string(&tokens[i])
                    .or(parse_ident(&tokens[i]))
                    .or(parse_usize(&tokens[i]))?;

                args.push(arg);
            }
        }
    }

    Ok((args, redirects))
}

/// Parse a command expression
///
/// A command expression can be a string, an identifier or a usize.
///
/// This parses the token that corresponds to the command name. For example, if there is a command and arguments such as ls -a ~, this is the parsing of the command name ls.
///
fn parse_command_expr(token: &Token) -> Result<flat_ast::Expr> {
    parse_string(token)
        .or(parse_ident(token))
        .or(parse_usize(token))
}

/// Parse a redirect
///
/// A redirect can be abbreviated or normal.
fn parse_redirect(tokens: &[Token]) -> Result<flat_ast::Redirect> {
    let len = tokens.len();

    if !(len == 2 || len == 3) {
        Err(Error::DUMMY)?;
    }

    let result = if len == 2 {
        parse_abbreviated_redirect(tokens[0..2].try_into().unwrap())?
    } else {
        parse_normal_redirect(tokens[0..3].try_into().unwrap())?
    };

    Ok(result)
}

/// Parse a normal redirect
///
/// A normal Redirect consists of three tokens, such as `1 > file.txt`.
///
fn parse_normal_redirect(tokens: &[Token; 3]) -> Result<flat_ast::Redirect> {
    let left = parse_fd(&tokens[0])?;

    let operator = match tokens[1] {
        Token::Gt => flat_ast::RecirectOperator::Gt,
        Token::Lt => flat_ast::RecirectOperator::Lt,
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected a redirect operator",
        ))?,
    };

    let right = parse_string(&tokens[2]).or(parse_ident(&tokens[2]).or(parse_fd(&tokens[2])))?;

    Ok(flat_ast::Redirect {
        left,
        right,
        operator,
    })
}

/// Parse an abbreviated redirect
///
/// A normal Redirect consists of three tokens, such as `1 > file.txt`.
///
/// In this case, it can be abbreviated as `> file.txt`.
///
fn parse_abbreviated_redirect(tokens: &[Token; 2]) -> Result<flat_ast::Redirect> {
    let (left, operator) = match tokens[0] {
        Token::Gt => (flat_ast::Expr::FD(1), flat_ast::RecirectOperator::Gt),
        Token::Lt => (flat_ast::Expr::FD(0), flat_ast::RecirectOperator::Lt),
        _ => Err(Error::new(
            ErrorKind::SyntaxError,
            "Expected a redirect operator",
        ))?,
    };

    let right = parse_string(&tokens[1]).or(parse_ident(&tokens[1]).or(parse_fd(&tokens[1])))?;

    Ok(flat_ast::Redirect {
        left,
        right,
        operator,
    })
}

/// Parse an assignment
///
/// The values of variables can only be strings or numbers.
///
/// In other words, they can only be Token::String or Token::USize.
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

// Parse a file descriptor literal
//
// Zero(0) and Positive FD values, will result in an error.
//
// fn parse_close_fd(token: &Token) -> Result<flat_ast::Expr> {
//     match token {
//         Token::FD(fd) => {
//             if fd > &0 {
//                 Err(Error::new(
//                     ErrorKind::SyntaxError,
//                     "Expected a file descriptor literal. File descriptor value cannot be positive",
//                 ))?
//             }

//             Ok(flat_ast::Expr::FD(*fd))
//         }

//         _ => Err(Error::new(
//             ErrorKind::SyntaxError,
//             "Expected a file descriptor literal",
//         ))?,
//     }
// }

/// Parse a file descriptor literal
///
/// Negative FD values, also known as CloseFD, will result in an error.
///
fn parse_fd(token: &Token) -> Result<flat_ast::Expr> {
    match token {
        Token::FD(fd) => {
            if fd < &0 {
                Err(Error::new(
                    ErrorKind::SyntaxError,
                    "Expected a file descriptor literal. File descriptor value cannot be negative",
                ))?
            }

            Ok(flat_ast::Expr::FD(*fd))
        }

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
    fn test_parse_pipe() {
        let tokens = [
            Token::String("A".to_string()),
            Token::Pipe,
            Token::String("B".to_string()),
            Token::String("C".to_string()),
            Token::Pipe,
            Token::String("D".to_string()),
        ];

        let pipe = parse_pipe(&tokens).unwrap();

        assert_eq!(pipe.commands.len(), 3);

        assert_eq!(
            pipe.commands[0].expr,
            flat_ast::Expr::String("A".to_string())
        );

        assert_eq!(
            pipe.commands[1].expr,
            flat_ast::Expr::String("B".to_string())
        );

        assert_eq!(
            pipe.commands[2].expr,
            flat_ast::Expr::String("D".to_string())
        );

        assert_eq!(pipe.commands[2].args.len(), 0);

        assert_eq!(pipe.commands[2].redirects.len(), 0);
    }

    #[test]
    fn test_parse_command_with_background(){
        let tokens = [
            Token::String("ls".to_string()),
            Token::Ampersand,
        ];

        let command = parse_command(&tokens).unwrap();

        assert_eq!(command.expr, flat_ast::Expr::String("ls".to_string()));

        assert_eq!(command.args.len(), 0);

        assert_eq!(command.redirects.len(), 0);

        assert_eq!(command.background, true);
    }

    #[test]
    fn test_parse_command() {
        let tokens = [
            Token::String("ls".to_string()),
            Token::String("-a".to_string()),
            Token::String("~".to_string()),
            Token::Gt,
            Token::String("file.txt".to_string()),
            Token::FD(2),
            Token::Gt,
            Token::String("file2.txt".to_string()),
        ];

        let command = parse_command(&tokens).unwrap();

        assert_eq!(command.expr, flat_ast::Expr::String("ls".to_string()));

        assert_eq!(command.args.len(), 2);

        assert_eq!(command.args[0], flat_ast::Expr::String("-a".to_string()));

        assert_eq!(command.args[1], flat_ast::Expr::String("~".to_string()));

        assert_eq!(command.redirects.len(), 2);

        assert_eq!(
            command.redirects[0],
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(1),
                right: flat_ast::Expr::String("file.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );

        assert_eq!(
            command.redirects[1],
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(2),
                right: flat_ast::Expr::String("file2.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );
    }

    #[test]
    fn test_parse_command_args_with_redirect() {
        let tokens = [
            Token::String("-a".to_string()),
            Token::String("~".to_string()),
            Token::Gt,
            Token::String("file.txt".to_string()),
            Token::FD(2),
            Token::Gt,
            Token::String("file2.txt".to_string()),
        ];

        let (args, redirects) = parse_command_args_with_redirect(&tokens).unwrap();

        assert_eq!(args.len(), 2);

        assert_eq!(args[0], flat_ast::Expr::String("-a".to_string()));

        assert_eq!(args[1], flat_ast::Expr::String("~".to_string()));

        assert_eq!(redirects.len(), 2);

        assert_eq!(
            redirects[0],
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(1),
                right: flat_ast::Expr::String("file.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );

        assert_eq!(
            redirects[1],
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(2),
                right: flat_ast::Expr::String("file2.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );
    }

    #[test]
    fn test_parse_command_expr() {
        let token = Token::String("ls".to_string());

        let expr = parse_command_expr(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::String("ls".to_string()));
    }

    #[test]
    fn test_parse_redirect() {
        let tokens = [
            Token::FD(1),
            Token::Gt,
            Token::String("file.txt".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(
            redirect,
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(1),
                right: flat_ast::Expr::String("file.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );

        let tokens = [Token::Gt, Token::String("file.txt".to_string())];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(
            redirect,
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(1),
                right: flat_ast::Expr::String("file.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );
    }

    #[test]
    fn test_parse_normal_redirect() {
        let tokens = [
            Token::FD(1),
            Token::Gt,
            Token::String("file.txt".to_string()),
        ];

        let redirect = parse_normal_redirect(&tokens).unwrap();

        assert_eq!(
            redirect,
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(1),
                right: flat_ast::Expr::String("file.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );
    }

    #[test]
    fn test_parse_abbreviated_redirect() {
        let tokens = [Token::Gt, Token::String("file.txt".to_string())];

        let redirect = parse_abbreviated_redirect(&tokens).unwrap();

        assert_eq!(
            redirect,
            flat_ast::Redirect {
                left: flat_ast::Expr::FD(1),
                right: flat_ast::Expr::String("file.txt".to_string()),
                operator: flat_ast::RecirectOperator::Gt
            }
        );
    }

    #[test]
    fn test_parse_assign() {
        let tokens = [
            Token::Ident("Hello".to_string()),
            Token::Assign,
            Token::String("World".to_string()),
        ];

        let assign = parse_assign(&tokens).unwrap();

        assert_eq!(
            assign,
            flat_ast::Assign {
                ident: flat_ast::Expr::Ident("Hello".to_string()),
                expr: flat_ast::Expr::String("World".to_string())
            }
        );
    }

    // #[test]
    // fn test_parse_close_fd() {
    //     let token = Token::FD(-1);

    //     let expr = parse_close_fd(&token).unwrap();

    //     assert_eq!(expr, flat_ast::Expr::FD(-1));
    // }

    #[test]
    fn test_parse_fd() {
        let token = Token::FD(0);

        let expr = parse_fd(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::FD(0));
    }

    #[test]
    fn test_parse_ident() {
        let token = Token::Ident("Hello".to_string());

        let expr = parse_ident(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::Ident("Hello".to_string()));
    }

    #[test]
    fn test_parse_string() {
        let token = Token::String("Hello".to_string());

        let expr = parse_string(&token).unwrap();

        assert_eq!(expr, flat_ast::Expr::String("Hello".to_string()));
    }
}
