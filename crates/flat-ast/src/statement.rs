use super::expr::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Statement {
    Command(Command),
    Assign(Assign),
}

impl Statement {
    pub fn to_json(&self, is_pretty: bool) -> String {
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

impl Command {
    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Assign {
    pub ident: Expr,
    pub expr: Expr,
}

impl Assign {
    pub fn to_json(&self, is_pretty: bool) -> String {
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
    pub operator: RecirectOperator,
}

impl Redirect {
    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RecirectOperator {
    Gt,
    Lt,
}

impl RecirectOperator {
    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}