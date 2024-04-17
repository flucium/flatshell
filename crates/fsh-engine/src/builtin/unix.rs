use super::super::State;
use fsh_common::*;
use std::ffi::OsStr;

/// Change the current directory
///
/// # Arguments
/// `p` - The path to change to
///
/// `state` - The current fsh state
///
/// # Errors
/// `Kind::NotFound` - If the path does not exist
///
/// `Kind::NotADirectory` - If the path is not a directory
pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S, state: &mut State) -> fsh_common::Result<()> {
    let current_path = state
        .current_dir()
        .canonicalize()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "invalid input path"))?;

    let path = analyze(&current_path, p)?;

    state.current_dir_mut().clear();

    state.current_dir_mut().push(path);

    Ok(())
}


/// Analyze the path
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
