use fsh_common::{Error, ErrorKind, Result};
use std::mem::ManuallyDrop;

/// ProcessHandler is a Vec that stores processes (specifically, std::process::*).
///
/// all process states are stored here.
///
/// Think of ProcessHandler as simply a Vector (Vec).
///
/// to empty ProcessHandler, you can use high-level operations such as deleting the ProcessHandler instance. ProcessHandler does not provide a remove method like Vec. This is intentional.
#[derive(Debug)]
pub struct ProcessHandler(Vec<(ManuallyDrop<std::process::Child>, bool)>);

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
    ///
    /// # Arguments
    /// - `ps` - The process to push
    /// - `is_background` - If the process is a background process
    ///
    /// # Returns
    /// The process id
    pub fn push(&mut self, ps: std::process::Child, is_background: bool) -> u32 {
        let pid = ps.id();

        self.0.push((ManuallyDrop::new(ps), is_background));

        pid
    }

    /// Pop a process from the handler
    pub fn pop(&mut self) -> Option<std::process::Child> {
        self.0.pop().map(|(ps, _)| ManuallyDrop::into_inner(ps))
    }

    /// Get a process from the handler
    pub fn get(&self, pid: u32) -> Option<&std::process::Child> {
        self.0
            .iter()
            .find(|(ps, _)| ps.id() == pid)
            .map(|(ps, _)| &**ps)
    }

    /// Get a mutable process from the handler
    pub fn get_mut(&mut self, pid: u32) -> Option<&mut std::process::Child> {
        self.0
            .iter_mut()
            .find(|(ps, _)| ps.id() == pid)
            .map(|(ps, _)| &mut **ps)
    }

    /// Get all processes from the handler
    pub fn entries(&self) -> Vec<&std::process::Child> {
        self.0.iter().map(|(ps, _)| &**ps).collect()
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
        self.0.iter_mut().try_for_each(|(ps, _)| -> Result<()> {
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

    /// Wait for all processes
    ///
    /// Processes specified in the background do not perform regular wait. treated as try_wait.
    ///
    /// foreground and Background processes appear to behave identically at first glance.
    ///
    /// however, background processes can be made to run completely in the background by combining them with nohup. additionally, such processes will be automatically closed after they terminate.
    ///
    /// Processes are dropped after wait, but this does not mean they are removed from the ProcessHandler. it simply refers to the dropping of the processes themselves.
    ///
    /// # Returns
    /// A vector of process id and exit status
    pub fn wait(&mut self) -> Vec<(u32, std::process::ExitStatus)> {
        let mut v = Vec::with_capacity(self.0.len());

        self.0.iter_mut().for_each(|(ps, is_background)| {
            if *is_background == true {
                // background process
                if let Ok(exitstatus) = ps.try_wait() {
                    if let Some(exitstatus) = exitstatus {
                        v.push((ps.id(), exitstatus));
                    }

                    unsafe {
                        ManuallyDrop::drop(ps);
                    }
                }
            } else {
                // foreground process
                if let Ok(status) = ps.wait() {
                    v.push((ps.id(), status));
                }

                unsafe {
                    ManuallyDrop::drop(ps);
                }
            }
        });

        v
    }
}

impl Drop for ProcessHandler {
    /// It simply refers to the dropping of the processes themselves.
    fn drop(&mut self) {
        self.0.iter_mut().for_each(|(ps, _)| unsafe {
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

        let pid = handler.push(ps, false);

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

        let pid = handler.push(ps, false);

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

        handler.push(ps1, false);

        handler.push(ps2, false);

        let v = handler.wait();

        assert_eq!(v.len(), 2);

        for (pid, status) in v {
            assert_eq!(status.success(), true);

            assert_eq!(handler.get(pid).unwrap().id(), pid);
        }
    }
}
