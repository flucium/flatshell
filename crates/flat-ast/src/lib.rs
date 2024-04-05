mod statement;
mod expr;
mod pipe;

use std::collections::VecDeque;

use serde::Serialize;

//pub use
pub use statement::*;
pub use expr::*;
pub use pipe::*;

#[derive(Debug, Clone, PartialEq,Serialize)]
pub enum FlatAst {
    Semicolon(VecDeque<FlatAst>),
    Pipe(Pipe),
    Statement(Statement),
}

impl FlatAst {
    pub fn new() -> Self {
        FlatAst::Semicolon(VecDeque::new())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            FlatAst::Semicolon(v) => v.is_empty(),
            _ => false,
        }
    }

    pub fn push_back(&mut self, ast: FlatAst) {
        match self {
            FlatAst::Semicolon(ref mut v) => v.push_back(ast),
            _ => panic!("Cannot push to non-sequence"),
        }
    }

    pub fn pop_front(&mut self) -> Option<FlatAst> {
        match self {
            FlatAst::Semicolon(ref mut v) => v.pop_front(),
            _ => panic!("Cannot pop from non-sequence"),
        }
    }


    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }    
}