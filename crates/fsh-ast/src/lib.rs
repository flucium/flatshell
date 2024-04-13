mod expr;
mod pipe;
mod statement;

use serde::Serialize;
use std::collections::VecDeque;

//pub use
pub use expr::*;
pub use pipe::*;
pub use statement::*;

pub trait FshAst {
    fn to_json(&self, is_pretty: bool) -> String;
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Ast {
    Semicolon(VecDeque<Ast>),
    Pipe(Pipe),
    Statement(Statement),
}

impl Ast {
    pub fn new() -> Self {
        Ast::Semicolon(VecDeque::new())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Ast::Semicolon(v) => v.is_empty(),
            _ => false,
        }
    }

    pub fn push_back(&mut self, ast: Ast) {
        match self {
            Ast::Semicolon(ref mut v) => v.push_back(ast),
            _ => panic!("Cannot push to non-sequence"),
        }
    }

    pub fn pop_front(&mut self) -> Option<Ast> {
        match self {
            Ast::Semicolon(ref mut v) => v.pop_front(),
            _ => panic!("Cannot pop from non-sequence"),
        }
    }
}

impl FshAst for Ast {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}
