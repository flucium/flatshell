use super::{Error, ErrorKind, Result};
use std::ffi::OsStr;

#[derive(Debug)]
pub struct Path(std::path::PathBuf);

impl Path {
    pub fn new<P: AsRef<OsStr> + ?Sized>(p: &P) -> Self {
        Self(std::path::PathBuf::from(p))
    }

    pub fn canonicalize(&mut self) -> Result<Self> {
        if self.0.is_dir() == false {
            Err(Error::new(ErrorKind::NotADirectory, "not a directory"))?
        }

        Ok(Self::new(
            self.0
                .canonicalize()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, "invalid input path"))?
                .as_path(),
        ))
    }

    pub fn change_dir<P: AsRef<OsStr> + ?Sized>(&mut self, p: &P) -> Result<&Self> {
        self.0 = analyze(&self.0, p)?;
        Ok(self)
    }


    pub fn as_path(&self) -> &std::path::Path {
        self.0.as_path()
    }

    pub fn as_str(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

impl From<&str> for Path {
    fn from(path: &str) -> Self {
        Self(std::path::PathBuf::from(path))
    }
}

impl From<String> for Path {
    fn from(path: String) -> Self {
        Self(std::path::PathBuf::from(path))
    }
}

pub fn exists(path: &str) -> bool {
    glob::glob(path).is_ok()
}

pub fn glob(pattern: &str) -> Result<Vec<Path>> {
    let paths = glob::glob(pattern)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "invalid input path"))?
        .map(|p| Path::new(&p.unwrap()))
        .collect();

    Ok(paths)
}

fn analyze<A: AsRef<OsStr> + ?Sized, B: AsRef<OsStr> + ?Sized>(
    current: &A,
    target: &B,
) -> Result<std::path::PathBuf> {
    let current = std::path::Path::new(current);

    let target = std::path::Path::new(target);

    if current.is_dir() == false {
        Err(Error::new(ErrorKind::NotADirectory, "not a directory"))?
    }

    let path = std::path::Path::new(current)
        .join(std::path::Path::new(target))
        .canonicalize()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "invalid input path"))?;

    if path.is_file() {
        Err(Error::new(ErrorKind::InvalidInput, "invalid input path"))?
    }

    Ok(path)
}
