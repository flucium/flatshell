use super::Command;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Pipe {
    pub commands: Vec<Command>,
}

impl Pipe {
    pub fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}
