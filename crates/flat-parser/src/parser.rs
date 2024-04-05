use flat_ast::FlatAst;
use flat_common::result::Result;

use super::lite_parser::{parse_assign, parse_command, parse_pipe};

use super::Lexer;

use super::token::Token;
use super::utils;

#[derive(Debug)]
pub struct Parser(Lexer, FlatAst);

impl Parser {
    pub fn new(input: &str) -> Self {
        Self(Lexer::new(input), FlatAst::new())
    }

    pub fn parse(&mut self) -> Result<FlatAst> {
        let mut tokens = Vec::new();

        while let Some(token) = self.0.next() {
            if token == Token::EOF {
                break;
            }

            tokens.push(token);
        }

        let entries = utils::recursion_split(&Token::Semicolon, &tokens);

        for tokens in entries {
            if tokens.is_empty() {
                continue;
            }

            if tokens.contains(&Token::Pipe) {
                self.1
                    .push_back(flat_ast::FlatAst::Pipe(parse_pipe(&tokens)?));

                continue;
            }

            if tokens.contains(&Token::Assign) && tokens.len() == 3 {
                self.1
                    .push_back(flat_ast::FlatAst::Statement(flat_ast::Statement::Assign(
                        parse_assign(&tokens.try_into().unwrap())?,
                    )));

                continue;
            }

            self.1
                .push_back(flat_ast::FlatAst::Statement(flat_ast::Statement::Command(
                    parse_command(&tokens)?,
                )));
        }

        Ok(self.1.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new("echo hello; echo world");

        let ast = parser.parse();

        assert_eq!(ast.is_ok(), true);
    }
}
