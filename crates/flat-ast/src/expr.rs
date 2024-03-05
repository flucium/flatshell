use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
    String(String),
    Ident(String),
    USize(usize),
    FD(i32),
}

impl Expr {
    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}
