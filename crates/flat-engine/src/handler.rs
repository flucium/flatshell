use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::mem::ManuallyDrop;

#[derive(Debug)]
pub struct Handler(Vec<ManuallyDrop<std::process::Child>>);

impl Handler {
    /// Create a new handler
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Create a new handler with a capacity
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Push a process to the handler
    pub fn push(&mut self, ps: std::process::Child) -> u32 {
        let pid = ps.id();

        self.0.push(ManuallyDrop::new(ps));

        pid
    }

    /// Pop a process from the handler
    pub fn pop(&mut self) -> Option<std::process::Child> {
        self.0.pop().map(|ps| ManuallyDrop::into_inner(ps))
    }

    /// Get a process from the handler
    pub fn get(&self, pid: u32) -> Option<&std::process::Child> {
        self.0.iter().find(|ps| ps.id() == pid).map(|ps| &**ps)
    }

    /// Get a mutable process from the handler
    pub fn get_mut(&mut self, pid: u32) -> Option<&mut std::process::Child> {
        self.0
            .iter_mut()
            .find(|ps| ps.id() == pid)
            .map(|ps| &mut **ps)
    }

    /// Get all processes from the handler
    pub fn entries(&self) -> Vec<&std::process::Child> {
        self.0.iter().map(|ps| &**ps).collect()
    }

    /// Check if the handler is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the length of the handler
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get the capacity of the handler
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Kill a process
    pub fn kill(&mut self, pid: u32) -> Result<()> {
        self.0.iter_mut().try_for_each(|ps| -> Result<()> {
            if ps.id() == pid {
                // try to kill the process
                let kill = ps.kill();

                // drop the process
                unsafe {
                    ManuallyDrop::drop(ps);
                }

                // check if the process was killed
                kill.map_err(|_| Error::new(ErrorKind::Failure, "Failed to kill the process."))?;

                return Ok(());
            }

            Ok(())
        })
    }

    /// Kill all processes
    pub fn kill_all(&mut self) -> Result<()> {
        self.0.iter_mut().try_for_each(|ps| -> Result<()> {
            // try to kill the process
            let kill = ps.kill();

            // drop the process
            unsafe {
                ManuallyDrop::drop(ps);
            }

            // check if the process was killed
            kill.map_err(|_| Error::new(ErrorKind::Failure, "Failed to kill the process."))?;

            Ok(())
        })
    }

    /// Wait for all processes to finish
    pub fn wait(&mut self) -> Vec<(u32, std::process::ExitStatus)> {
        let mut v = Vec::with_capacity(self.0.len());

        self.0.iter_mut().for_each(|ps| {
            if let Ok(status) = ps.wait() {
                v.push((ps.id(), status));
            }

            unsafe {
                ManuallyDrop::drop(ps);
            }
        });

        v
    }

    /// Wait for all processes to finish and leak them
    pub unsafe fn wait_and_leak(&mut self) -> Vec<(u32, std::process::ExitStatus)> {
        let mut v = Vec::with_capacity(self.0.len());

        self.0.iter_mut().for_each(|ps| {
            if let Ok(status) = ps.wait() {
                v.push((ps.id(), status));
            }
        });

        v
    }
}
