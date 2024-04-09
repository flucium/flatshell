use super::{Pipe, ProcessHandler, ShVars};

/// The state of the shell
/// 
/// By keeping the state outside the body of the runtime or the evaluation function, state management becomes easier. 
/// 
/// there is no other purpose for this. therefore, this structure only provides(implementation) a function (new) for creating a State.
/// 
/// If you already have another structure that the State structure needs, and you want to inherit from them, you can use From.
#[derive(Debug)]
pub struct State {
    pub(super) vars: ShVars,
    pub(super) handler: ProcessHandler,
    pub(super) pipe: Pipe,
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
}

impl From<ShVars> for State {
    fn from(vars: ShVars) -> Self {
        Self {
            vars,
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
        }
    }
}

impl From<ProcessHandler> for State {
    fn from(handler: ProcessHandler) -> Self {
        Self {
            vars: ShVars::new(),
            handler,
            pipe: Pipe::new(),
        }
    }
}

impl From<Pipe> for State {
    fn from(pipe: Pipe) -> Self {
        Self {
            vars: ShVars::new(),
            handler: ProcessHandler::new(),
            pipe,
        }
    }
}

impl From<(ShVars, ProcessHandler)> for State {
    fn from((vars, handler): (ShVars, ProcessHandler)) -> Self {
        Self {
            vars,
            handler,
            pipe: Pipe::new(),
        }
    }
}

impl From<(ShVars, Pipe)> for State {
    fn from((vars, pipe): (ShVars, Pipe)) -> Self {
        Self {
            vars,
            handler: ProcessHandler::new(),
            pipe,
        }
    }
}

impl From<(ProcessHandler, Pipe)> for State {
    fn from((handler, pipe): (ProcessHandler, Pipe)) -> Self {
        Self {
            vars: ShVars::new(),
            handler,
            pipe,
        }
    }
}

impl From<(ShVars, ProcessHandler, Pipe)> for State {
    fn from((vars, handler, pipe): (ShVars, ProcessHandler, Pipe)) -> Self {
        Self {
            vars,
            handler,
            pipe,
        }
    }
}