mod statement;
mod expr;
mod pipe;

//pub use
pub use statement::*;
pub use expr::*;
pub use pipe::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FlatAst {
    Semicolon(Vec<FlatAst>),
    Pipe(Vec<FlatAst>),
    Expr(Expr),
    Statement(Statement),
}

impl FlatAst {
    pub fn new() -> Self {
        FlatAst::Semicolon(Vec::new())
    }
    
}
