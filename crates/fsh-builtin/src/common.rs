use std::{
    collections::HashMap,
    io::{stdout, Write},
    process,
};

pub fn exit(code: i32) {
    process::exit(code)
}

pub fn abort() {
    process::abort()
}

pub fn printenv(key: impl Into<String>, vars: HashMap<&String, &String>) -> fsh_common::Result<()> {
    let key = key.into();

    let mut stdout = stdout().lock();

    if key.len() == 0 {
        vars.iter().try_for_each(|(key, value)| {
            stdout
                .write_fmt(format_args!("{}={}\n", key, value))
                .map_err(|_| {
                    fsh_common::Error::new(
                        fsh_common::ErrorKind::Internal,
                        "failed to write to stdout",
                    )
                })
        })?;
    } else if let Some(value) = vars.get(&key) {
        stdout
            .write_fmt(format_args!("{}={}\n", key, value))
            .map_err(|_| fsh_common::Error::INTERNAL)?;
    } else {
        stdout
            .write_fmt(format_args!(""))
            .map_err(|_| fsh_common::Error::INTERNAL)?;
    }

    Ok(())
}
