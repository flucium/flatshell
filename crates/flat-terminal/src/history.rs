use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

#[derive(Debug)]
pub struct History {
    pub entries: Vec<String>,
    pub current: usize,
}

impl History {
    /// Create a new history
    ///
    /// # Example
    /// ```
    /// use flat_terminal::History;
    ///
    /// let mut history = History::new();
    ///
    /// history.push("ls");
    ///
    /// assert_eq!(history.entries, vec!["ls"]);
    /// ```
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
        }
    }

    /// Open a history file
    ///
    /// # Example
    /// ```
    /// use std::path::Path;
    /// use flat_terminal::History;
    ///
    /// // let history = History::open(Path::new(""));
    ///
    /// // assert!(history.is_ok());
    /// ```
    pub fn open(path: &Path) -> Result<Self> {
        let mut file = match fs::File::open(path) {
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Err(Error::new(ErrorKind::NotFound, "File not found"))?,
                io::ErrorKind::PermissionDenied => {
                    Err(Error::new(ErrorKind::PermissionDenied, "Permission denied"))?
                }
                _ => Err(Error::new(ErrorKind::Other, "Unknown error"))?,
            },
            Ok(file) => file,
        };

        let mut buf = Vec::new();

        file.read_to_end(&mut buf)
            .map_err(|err| Error::new(ErrorKind::Other, &err.to_string()))?;

        let entries = String::from_utf8(buf)
            .map_err(|err| Error::new(ErrorKind::Other, &err.to_string()))?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok(Self {
            entries,
            current: 0,
        })
    }

    /// Save the history to a file
    ///
    /// # Example
    /// ```
    /// use std::path::Path;
    /// use flat_terminal::History;
    ///
    /// // let history = History::new();
    ///
    /// // history.save(Path::new("history.txt"));
    ///
    /// // assert!(Path::new("history.txt").exists());
    /// ```
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file =
            fs::File::create(path).map_err(|err| Error::new(ErrorKind::Other, &err.to_string()))?;

        for entry in &self.entries {
            file.write_all(entry.as_bytes())
                .map_err(|err| Error::new(ErrorKind::Other, &err.to_string()))?;

            file.write_all(b"\n")
                .map_err(|err| Error::new(ErrorKind::Other, &err.to_string()))?;
        }

        Ok(())
    }

    /// Add a new entry to the history
    ///
    /// # Example
    /// ```
    /// use flat_terminal::History;
    ///
    /// let mut history = History::new();
    ///
    /// history.push("ls");
    ///
    /// assert_eq!(history.entries, vec!["ls"]);
    /// ```
    pub fn push(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
        self.current = self.entries.len();
    }

    /// Get the previous entry in the history: (cursor up)
    ///
    /// # Example
    /// ```
    /// use flat_terminal::History;
    ///
    /// let mut history = History::new();
    ///
    /// history.push("ls");
    ///
    /// assert_eq!(history.prev(), Some(&"ls".to_string()));
    /// ```
    pub fn prev(&mut self) -> Option<&String> {
        if self.current > 0 {
            self.current -= 1;
            self.entries.get(self.current)
        } else {
            None
        }
    }

    /// Get the next entry in the history: (cursor down)
    ///
    /// # Example
    /// ```
    /// use flat_terminal::History;
    ///
    /// let mut history = History::new();
    ///
    /// history.push("ls");
    ///
    /// assert_eq!(history.next(), None);
    /// ```
    pub fn next(&mut self) -> Option<&String> {
        if self.current < self.entries.len() {
            self.current += 1;
            self.entries.get(self.current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_history_new() {
        let history = super::History::new();

        assert_eq!(history.entries.len(), 0);
    }

    #[test]
    fn test_history_push() {
        let mut history = super::History::new();

        history.push("ls");

        history.push("cd");

        assert_eq!(history.entries, vec!["ls", "cd"]);
    }

    #[test]
    fn test_history_prev() {
        let mut history = super::History::new();

        history.push("ls");

        history.push("cd");

        assert_eq!(history.prev(), Some(&"cd".to_string()));

        assert_eq!(history.prev(), Some(&"ls".to_string()));

        assert_eq!(history.prev(), None);
    }

    #[test]
    fn test_history_next() {
        let mut history = super::History::new();

        history.push("ls");

        history.push("cd");

        assert_eq!(history.prev(), Some(&"cd".to_string()));

        assert_eq!(history.next(), None);
    }
}
