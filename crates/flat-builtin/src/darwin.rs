use flat_common::error::Error;
use flat_common::result::Result;
use std::env::{set_current_dir, set_var};
use std::path::Path;

pub fn cd(path: &Path) -> Result<()> {
    if path.exists() {
        set_current_dir(&path).map_err(|_| Error::DUMMY)?;

        set_var("PWD", path);

        Ok(())
    } else {
        Err(Error::DUMMY)
    }
}
