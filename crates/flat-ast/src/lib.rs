mod statement;
mod expr;
mod pipe;

use serde::Serialize;

//pub use
pub use statement::*;
pub use expr::*;
pub use pipe::*;

#[derive(Debug, Clone, PartialEq,Serialize)]
pub enum FlatAst {
    Semicolon(Vec<FlatAst>),
    Pipe(Pipe),
    Statement(Statement),
}

impl FlatAst {
    pub fn new() -> Self {
        FlatAst::Semicolon(Vec::new())
    }

    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }    
}