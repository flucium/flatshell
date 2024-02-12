use super::expr::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Command(Command),
    Assign(Assign),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub expr: Expr,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub ident: String,
    pub expr: Expr,
}
