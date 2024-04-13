use super::{pipe::*, process_handler::*, sh_vars::*};

#[derive(Debug)]
pub struct State {
    vars: ShVars,
    handler: ProcessHandler,
    pipe: Pipe,
}

impl State {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            vars: ShVars::new(),
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
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

    pub fn pipe(&self) -> &Pipe {
        &self.pipe
    }

    pub fn pipe_mut(&mut self) -> &mut Pipe {
        &mut self.pipe
    }
}
