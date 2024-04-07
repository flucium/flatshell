use std::{fs, io::Write, path};

use flat_engine::{eval, ShVars, State};
use flat_parser::Parser;
use flat_terminal::Terminal;

// -- <Bat Code> --
fn open_profile() -> ShVars {
    match ShVars::open(path::Path::new("./temp/profile.fsh")) {
        Ok(shvars) => shvars,
        Err(err) => {
            if err.kind() == &flat_common::error::ErrorKind::NotFound {
                let mut string = String::new();

                string.push_str("$FSH_PROMPT = \"> \";\n");

                fs::File::create("./temp/profile.fsh")
                    .unwrap()
                    .write_all(string.as_bytes())
                    .unwrap();

                open_profile()
            } else {
                panic!("{}", err.message());
            }
        }
    }
}
// -- <Bat Code> --

fn repl() {
    let shvars = open_profile();

    let mut state = State::from(shvars);

    let mut terminal = Terminal::new();

    // -- <Bat Code> --
    let mut buffer = String::new();
    for var in state.vars().entries() {
        buffer.push_str(&format!("{} = {}\n", var.0, var.1));

        buffer.push(';');
    }

    let ast = match Parser::new(&buffer).parse() {
        Ok(ast) => ast,
        Err(_) => panic!("あとでね"),
    };

    if let Err(_) = eval(ast, &mut state) {
        panic!("あとでね2");
    }

    terminal.set_prompt(state.vars().get("FSH_PROMPT").unwrap_or_default());

    // -- </Bat Code> --

    loop {
        match terminal.read_line() {
            Err(e) => {
                panic!("Error: {}", e.message());
            }
            Ok(line) => {
                let ast = match Parser::new(&line).parse() {
                    Err(e) => {
                        panic!("Error: {}", e.message());
                    }
                    Ok(ast) => ast,
                };

                if let Err(err) = eval(ast, &mut state) {
                    panic!("{}", err.message());
                }
            }
        }
    }
}

fn main() {
    repl();
}
