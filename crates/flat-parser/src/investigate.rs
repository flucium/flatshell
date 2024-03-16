// Investigate (Error investigation).
// Based on parse errors, the syntax(tokens) will be investigated for errors.
//  specific error locations will be examined and raised as strings in a user-friendly manner.

/*
    ToDo: Investigation of Redirects is possible, but investigating Redirects within commands is not. This needs to be implemented.
*/

use super::token::Token;
use flat_common::error::{Error, ErrorKind};

pub(super) fn investigate_command(tokens: &[Token]) -> Option<Error> {
    let tokens_len = tokens.len();

    if tokens_len == 0 {
        return Some(Error::new(
            ErrorKind::SyntaxError,
            "Tokens must be at least 1 len long.",
        ));
    }

    if !matches!(
        tokens[0],
        Token::Ident(_) | Token::String(_) | Token::USize(_)
    ) {
        return Some(new_error(
            tokens,
            0,
            "The first token of a command must be an string, identifier or a number.",
        ));
    }

    let mut skip_count = 0;

    for i in 1..tokens_len {
        if skip_count > 0 {
            skip_count -= 1;

            continue;
        }

        if matches!(tokens[i], Token::Gt | Token::Lt) {
            if i + 1 < tokens_len {
                investigate_redirect(&tokens[i..i + 1])?;
                
                skip_count = 1;

                continue;
            }else{
                return Some(new_error(
                    tokens,
                    i,
                    "The middle token of a command must be an string, identifier, number.",
                ));
            }
        }

        if matches!(tokens[i], Token::FD(_)) {
            if i + 2 < tokens_len {
                investigate_redirect(&tokens[i..i + 2])?;

                skip_count = 2;

                continue;
            }else{
                return Some(new_error(
                    tokens,
                    i,
                    "The middle token of a command must be an string, identifier, number.",
                ));
            }
        }

        if !matches!(
            tokens[i],
            Token::Ident(_) | Token::String(_) | Token::USize(_)
        ) {
            return Some(new_error(
                tokens,
                i,
                "The middle token of a command must be an string, identifier, number.",
            ));
        }
    }

    None
}

pub(super) fn investigate_redirect(tokens: &[Token]) -> Option<Error> {
    let tokens_len = tokens.len();

    if tokens_len == 2 {
        if !matches!(tokens[0], Token::Gt | Token::Lt) {
            return Some(new_error(
                tokens,
                0,
                "The left-hand side of a redirect must be a redirect operator.",
            ));
        }

        if !matches!(
            tokens[1],
            Token::String(_) | Token::Ident(_) | Token::USize(_) | Token::FD(_)
        ) {
            return Some(new_error(
                tokens,
                1,
                "The right-hand side of a redirect must be a string.",
            ));
        }
    } else if tokens_len == 3 {
        if !matches!(tokens[0], Token::FD(_)) {
            return Some(new_error(
                tokens,
                0,
                "The left-hand side of a redirect must be a file descriptor.",
            ));
        }

        if !matches!(tokens[1], Token::Gt | Token::Lt) {
            return Some(new_error(
                tokens,
                1,
                "The middle token of a redirect must be a redirect operator.",
            ));
        }

        if !matches!(
            tokens[2],
            Token::String(_) | Token::Ident(_) | Token::USize(_) | Token::FD(_)
        ) {
            return Some(new_error(
                tokens,
                2,
                "The right-hand side of a redirect must be a string.",
            ));
        }
    } else {
        return Some(Error::new(
            ErrorKind::SyntaxError,
            "Token length is not equal to 2 or 3.",
        ));
    }

    None
}

/// Investigate the tokens for errors.
pub(super) fn investigate_assignment(tokens: &[Token]) -> Option<Error> {
    if tokens.len() != 3 {
        return Some(Error::new(
            ErrorKind::SyntaxError,
            "Token length is not equal to 3.",
        ));
    }

    if !matches!(tokens[0], Token::Ident(_)) {
        return Some(new_error(
            tokens,
            0,
            "The left-hand side of an assignment must be an identifier.",
        ));
    }

    if !matches!(tokens[1], Token::Assign) {
        return Some(new_error(
            tokens,
            1,
            "The middle token of an assignment must be an assign operator.",
        ));
    }

    if !matches!(tokens[2], Token::String(_) | Token::USize(_)) {
        return Some(new_error(
            tokens,
            2,
            "The right-hand side of an assignment must be a string or a number.",
        ));
    }

    None
}

/// Investigate the tokens for errors.
///
/// # Returns
/// returns an Error.
///
/// # Arguments
/// - `tokens` - A slice of tokens.
/// - `position` - The position of the error.
/// - `message` - The error message.
///
#[inline]
fn new_error(tokens: &[Token], position: usize, message: &str) -> Error {
    let (msg, tag) = new_error_message(tokens, position, message);

    if tag == 0 {
        Error::new(ErrorKind::Internal, &msg)
    } else {
        Error::new(ErrorKind::SyntaxError, &msg)
    }
}

/// Create a new error message.
///
/// # Arguments
/// - `tokens` - A slice of tokens.
/// - `position` - The position of the error.
/// - `message` - The error message.
///
/// # Returns
/// A tuple containing the error message and a tag.
///
/// The tag is used to determine if the error is an internal error or a syntax error.
/// - `Tag 0: Internal error.`
///
/// - `Tag 1: Syntax error.`
///
#[inline]
fn new_error_message(tokens: &[Token], position: usize, message: &str) -> (String, u8) {
    let mut buffer = String::new();

    let tokens_len = tokens.len();

    if position + 1 > tokens_len {
        return (String::from("Internal error"), 0);
    }

    for token in tokens {
        buffer.push_str(&format!("{} ", token));
    }

    buffer.push('\n');

    for i in 0..tokens.len() {
        if i == position {
            buffer.push_str(&format!("↑--- Error here:{}", message));

            break;
        } else {
            for _ in 0..tokens[i].len() + 1 {
                buffer.push(' ');
            }
        }
    }

    (buffer, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_investigate_command() {
        // case 1
        assert_eq!(
            investigate_command(&vec![Token::String(String::from("ls"))]),
            None
        );

        // case 2
        assert_eq!(
            investigate_command(&vec![
                Token::String(String::from("ls")),
                Token::String(String::from("-a")),
                Token::String(String::from("~")),
            ]),
            None
        );

        // case 3
        assert_eq!(
            investigate_command(&vec![
                Token::String(String::from("ls")),
                Token::Semicolon,
            ]),
            Some(Error::new(
                ErrorKind::SyntaxError,
                "ls ; \n   ↑--- Error here:The middle token of a command must be an string, identifier, number."
            ))
        );

        // case 4
        assert_eq!(
            investigate_command(&vec![]),
            Some(Error::new(
                ErrorKind::SyntaxError,
                "Tokens must be at least 1 len long."
            ))
        );
    }

    #[test]
    fn test_investigate_command_with_redirect() {
        // case 1
        assert_eq!(
            investigate_command(&vec![
                Token::String(String::from("ls")),
                Token::Gt,
                Token::String(String::from("test.txt")),
            ]),
            None
        );
    }

    // #[test]
    // fn test_investigate_redirect() {
    //     // case 1
    //     assert_eq!(
    //         investigate_redirect(&vec![Token::Gt, Token::String(String::from("a"))]),
    //         None
    //     );

    //     // case 2
    //     assert_eq!(
    //         investigate_redirect(&vec![
    //             Token::FD(1),
    //             Token::Gt,
    //             Token::String(String::from("a")),
    //         ]),
    //         None
    //     );

    //     // case 3
    //     assert_eq!(
    //         investigate_redirect(&vec![
    //             Token::Semicolon,
    //             Token::Gt,
    //             Token::String(String::from("a")),
    //         ]),
    //         Some(Error::new(
    //             ErrorKind::SyntaxError,
    //             "; > a \n↑--- Error here:The left-hand side of a redirect must be a file descriptor."
    //         ))
    //     );

    //     // case 4
    //     assert_eq!(
    //         investigate_redirect(&vec![
    //             Token::FD(1),
    //             Token::Gt,
    //             Token::String(String::from("a")),
    //             Token::String(String::from("b")),
    //         ]),
    //         Some(Error::new(
    //             ErrorKind::SyntaxError,
    //             "Token length is not equal to 2 or 3."
    //         ))
    //     );

    //     // case 5
    //     assert_eq!(
    //         investigate_redirect(&vec![]),
    //         Some(Error::new(
    //             ErrorKind::SyntaxError,
    //             "Token length is not equal to 2 or 3."
    //         ))
    //     );
    // }

    #[test]
    fn test_investigate_assignment() {
        // case 1
        assert_eq!(
            investigate_assignment(&vec![
                Token::Ident(String::from("a")),
                Token::Assign,
                Token::String(String::from("b")),
            ]),
            None
        );

        // case 2
        assert_eq!(
            investigate_assignment(&vec![
                Token::Ident(String::from("a")),
                Token::Assign,
                Token::USize(1),
            ]),
            None
        );

        // case 3
        assert_eq!(
            investigate_assignment(&vec![
                Token::Ident(String::from("a")),
                Token::Assign,
                Token::Semicolon,
            ]),
            Some(Error::new(
                ErrorKind::SyntaxError,
                "a = ; \n    ↑--- Error here:The right-hand side of an assignment must be a string or a number."
            ))
        );

        // case 4
        assert_eq!(
            investigate_assignment(&vec![]),
            Some(Error::new(
                ErrorKind::SyntaxError,
                "Token length is not equal to 3."
            ))
        );
    }

    #[test]
    fn test_new_error() {
        // case 1
        assert_eq!(
            new_error(&vec![
                Token::Ident(String::from("a")),
                Token::Assign,
                Token::Semicolon,
            ], 2, "The right-hand side of an assignment must be a string or a number."),
            Error::new(
                ErrorKind::SyntaxError,
                "a = ; \n    ↑--- Error here:The right-hand side of an assignment must be a string or a number."
            )
        );

        // case 2
        assert_eq!(
            new_error(
                &vec![
                    Token::Ident(String::from("a")),
                    Token::Assign,
                    Token::String(String::from("b")),
                ],
                1000,
                "ERROR MESSAGE"
            ),
            Error::new(ErrorKind::Internal, "Internal error")
        );
    }

    #[test]
    fn test_new_error_message() {
        // case 1
        assert_eq!(
            new_error_message(
                &vec![
                    Token::Ident(String::from("a")),
                    Token::Assign,
                    Token::Semicolon,
                ],
                2,
                "The right-hand side of an assignment must be a string or a number."
            ),
            (
                String::from("a = ; \n    ↑--- Error here:The right-hand side of an assignment must be a string or a number."),
                1
            )
        );

        // case 2
        assert_eq!(
            new_error_message(
                &vec![
                    Token::Ident(String::from("a")),
                    Token::Assign,
                    Token::String(String::from("b")),
                ],
                1000,
                "ERROR MESSAGE"
            ),
            (String::from("Internal error"), 0)
        );
    }
}
