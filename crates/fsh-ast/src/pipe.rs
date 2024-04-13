use super::{FshAst, Command};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Pipe {
    commands: VecDeque<Command>,
}

impl Pipe {
    pub fn new() -> Self {
        Pipe {
            commands: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn push_back(&mut self, command: Command) {
        self.commands.push_back(command);
    }

    pub fn pop_front(&mut self) -> Option<Command> {
        self.commands.pop_front()
    }
}

impl From<VecDeque<Command>> for Pipe {
    fn from(commands: VecDeque<Command>) -> Self {
        Pipe { commands }
    }
}

impl From<&[Command]> for Pipe {
    fn from(commands: &[Command]) -> Self {
        Pipe {
            commands: commands.iter().cloned().collect(),
        }
    }
}

impl FshAst for Pipe {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}
