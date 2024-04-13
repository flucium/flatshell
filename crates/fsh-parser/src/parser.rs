use super::lexer::Lexer;
use super::lite_parser::{parse_assign, parse_command, parse_pipe};
use super::token::Token;
use super::utils::recursion_split;
use fsh_ast::*;

use fsh_common::Result;

#[derive(Debug)]
pub struct Parser(Lexer, Ast);

impl Parser {
    pub fn new(input: &str) -> Self {
        Self(Lexer::new(input), Ast::new())
    }

    pub fn parse(&mut self) -> Result<Ast> {
        let mut tokens = Vec::new();

        while let Some(token) = self.0.next() {
            if token == Token::EOF {
                break;
            }

            tokens.push(token);
        }

        let entries = recursion_split(&Token::Semicolon, &tokens);

        for tokens in entries {
            if tokens.is_empty() {
                continue;
            }

            if tokens.contains(&Token::Pipe) {
                self.1.push_back(Ast::Pipe(parse_pipe(&tokens)?));

                continue;
            }

            if tokens.contains(&Token::Assign) && tokens.len() == 3 {
                self.1
                    .push_back(Ast::Statement(Statement::Assign(parse_assign(
                        &tokens.try_into().unwrap(),
                    )?)));

                continue;
            }

            self.1
                .push_back(Ast::Statement(Statement::Command(parse_command(&tokens)?)));
        }

        Ok(self.1.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut parser =
            Parser::new("$A = HELLO; echo hello; echo world; $var=hello; echo $var; $var = 10");

        let ast = parser.parse();

        assert_eq!(ast.is_ok(), true);
    }
}
