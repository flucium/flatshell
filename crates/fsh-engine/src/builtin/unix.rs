use super::super::State;
use std::{ffi::OsStr, path};

pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S, state: &mut State) -> fsh_common::Result<()> {
    state.set_current_dir(path::Path::new(p))?;

    Ok(())
}
