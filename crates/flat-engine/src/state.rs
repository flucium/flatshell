use super::{Pipe, ProcessHandler, ShVars};

#[derive(Debug)]
pub struct State {
    vars: ShVars,
    handler: ProcessHandler,
    stdin: Pipe,
    stdout: Pipe,
    stderr: Pipe,
}

impl State {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            vars: ShVars::new(),
            handler: ProcessHandler::new(),
            stdin: Pipe::new(),
            stdout: Pipe::new(),
            stderr: Pipe::new(),
            
        }
    }

    pub fn vars(&self) -> &ShVars {
        &self.vars
    }

    pub fn vars_mut(&mut self) -> &mut ShVars {
        &mut self.vars
    }

    pub fn handler(&self) -> &ProcessHandler {
        &self.handler
    }

    pub fn handler_mut(&mut self) -> &mut ProcessHandler {
        &mut self.handler
    }

    pub fn stdin(&self) -> &Pipe {
        &self.stdin
    }

    pub fn stdin_mut(&mut self) -> &mut Pipe {
        &mut self.stdin
    }

    pub fn stdout(&self) -> &Pipe {
        &self.stdout
    }

    pub fn stdout_mut(&mut self) -> &mut Pipe {
        &mut self.stdout
    }

    pub fn stderr(&self) -> &Pipe {
        &self.stderr
    }

    pub fn stderr_mut(&mut self) -> &mut Pipe {
        &mut self.stderr
    }

}
