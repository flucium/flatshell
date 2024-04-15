use super::super::State;
use std::{ffi::OsStr, path};

pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S, state: &mut State) -> fsh_common::Result<()> {
    let path = state
        .current_dir()?
        .join(path::Path::new(p))
        .canonicalize()
        .unwrap();

    state.set_current_dir(&path)?;

    Ok(())
}
