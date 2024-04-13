use std::{
    env::{set_current_dir, set_var},
    ffi::OsStr,
    path::Path,
};

pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S) -> fsh_common::Result<()> {
    let path = Path::new(p);

    let path = if path.exists() { path } else { Path::new("/") };

    if path.is_dir() == false {
        Err(fsh_common::Error::new(
            fsh_common::ErrorKind::NotFound,
            "not a directory",
        ))?
    }

    set_current_dir(path).map_err(|_| fsh_common::Error::INTERNAL)?;

    set_var("PWD", path);

    Ok(())
}
