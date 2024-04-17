use std::path::{Path, PathBuf};

use super::{pipe::*, process_handler::*};

#[derive(Debug)]
pub struct State {
    handler: ProcessHandler,
    pipe: Pipe,
    current_dir: PathBuf,
}

impl State {
    pub fn new() -> Self {
        Self {
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
            current_dir:PathBuf::new(),
        }
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

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    pub fn current_dir_mut(&mut self) -> &mut PathBuf {
        &mut self.current_dir
    }

}
