use std::path::{Path, PathBuf};

use fsh_common::{Error, Result};

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
            current_dir: PathBuf::new(),
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

    pub fn current_dir(&self) -> Result<&Path> {
        if self.current_dir.exists() == false {
            Err(Error::new(
                fsh_common::ErrorKind::NotFound,
                "no such file or directory",
            ))?
        }

        if self.current_dir.is_dir() == false {
            Err(Error::new(
                fsh_common::ErrorKind::NotADirectory,
                "not a directory",
            ))?
        }

        Ok(&self.current_dir)
    }

    pub fn set_current_dir(&mut self, path: &Path)->Result<()> {
        
        if path.exists() == false{
            Err(Error::new(
                fsh_common::ErrorKind::NotFound,
                "no such file or directory",
            ))?
        }

        if path.is_dir() == false{
            Err(Error::new(
                fsh_common::ErrorKind::NotADirectory,
                "not a directory",
            ))?
        }

        self.current_dir = path.to_path_buf();
        
        Ok(())
    }
}
