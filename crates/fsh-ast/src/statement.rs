use super::{FshAst, expr::*};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Assign {
    pub ident: Expr,
    pub expr: Expr,
}

impl FshAst for Assign {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Redirect {
    pub left: Expr,
    pub right: Expr,
    pub operator: RedirectOperator,
}

impl FshAst for Redirect {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedirectOperator {
    Gt,
    Lt,
}

impl FshAst for RedirectOperator {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Command {
    pub expr: Expr,
    pub args: Vec<Expr>,
    pub redirects: Vec<Redirect>,
    pub background: bool,
}

impl FshAst for Command {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Statement {
    Command(Command),
    Assign(Assign),
}

impl FshAst for Statement {
    fn to_json(&self, is_pretty: bool) -> String {
        match self {
            Statement::Command(command) => command.to_json(is_pretty),
            Statement::Assign(assign) => assign.to_json(is_pretty),
        }
    }
}