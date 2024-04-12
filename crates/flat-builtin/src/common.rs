use std::{
    io::{stdout, Write},
    process,
};

pub fn exit(code: i32) {
    process::exit(code)
}

pub fn abort() {
    process::abort()
}

pub fn printenv(key: &str, vars:Vec<(&String, &String)>) {
    let mut stdout = stdout().lock();

    if key.len() == 0 {
        for (key, value) in vars.iter() {
            stdout
                .write_fmt(format_args!("{}={}\n", key, value))
                .unwrap();
        }

        return;
    }

    for (k, v) in vars.iter() {
        if k == &key {
            stdout.write_fmt(format_args!("{}={}\n", k, v)).unwrap();
            return;
        }
    }
}
