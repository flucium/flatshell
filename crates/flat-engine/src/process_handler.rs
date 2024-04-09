use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::mem::ManuallyDrop;

#[derive(Debug)]
pub struct ProcessHandler(Vec<ManuallyDrop<std::process::Child>>);

impl ProcessHandler {
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
            }

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
}

impl Drop for ProcessHandler {
    fn drop(&mut self) {
        self.0.iter_mut().for_each(|ps| unsafe {
            ManuallyDrop::drop(ps);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_handler() {
        let handler = ProcessHandler::new();

        assert_eq!(handler.is_empty(), true);

        assert_eq!(handler.len(), 0);
    }

    #[test]
    fn test_process_handler_push() {
        let mut handler = ProcessHandler::new();

        let ps = std::process::Command::new("echo")
            .arg("Hello")
            .spawn()
            .unwrap();

        let pid = handler.push(ps);

        assert_eq!(handler.len(), 1);

        assert_eq!(handler.get(pid).unwrap().id(), pid);
    }

    #[test]
    fn test_process_handler_pop() {
        let mut handler = ProcessHandler::new();

        let ps = std::process::Command::new("echo")
            .arg("Hello")
            .spawn()
            .unwrap();

        let pid = handler.push(ps);

        assert_eq!(handler.len(), 1);

        assert_eq!(handler.pop().unwrap().id(), pid);

        assert_eq!(handler.len(), 0);
    }

    #[test]
    fn test_process_handler_wait() {
        let mut handler = ProcessHandler::new();

        let ps1 = std::process::Command::new("echo")
            .arg("hello")
            .spawn()
            .unwrap();

        let ps2 = std::process::Command::new("echo")
            .arg("world")
            .spawn()
            .unwrap();

        handler.push(ps1);

        handler.push(ps2);

        let v = handler.wait();

        assert_eq!(v.len(), 2);

        for (pid, status) in v {
            assert_eq!(status.success(), true);

            assert_eq!(handler.get(pid).unwrap().id(), pid);
        }
    }
}
