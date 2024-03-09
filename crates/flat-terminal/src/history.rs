use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

pub struct History {
    pub entries: Vec<String>,
    pub current: usize,
}

impl History {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
        }
    }

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

    pub fn push(&mut self, entry: String) {
        self.entries.push(entry);
        self.current = self.entries.len();
    }

    pub fn prev(&mut self) -> Option<&String> {
        if self.current > 0 {
            self.current -= 1;
            self.entries.get(self.current)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.current < self.entries.len() {
            self.current += 1;
            self.entries.get(self.current)
        } else {
            None
        }
    }
}
