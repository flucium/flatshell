mod statement;
mod expr;

//pub use
pub use statement::*;
pub use expr::*;

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
