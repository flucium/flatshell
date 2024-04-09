mod pipe;
mod process_handler;
mod sh_vars;
mod state;
use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
pub use pipe::*;
pub use process_handler::*;
pub use sh_vars::*;
pub use state::*;
use std::{
    os::unix::io::{AsRawFd, FromRawFd},
    process,
};

/*
    ToDo: Redirect, Builtin Command
*/

pub fn eval(ast: flat_ast::FlatAst, state: &mut State) -> Result<()> {
    match ast.to_owned() {
        flat_ast::FlatAst::Semicolon(mut semicolon) => {
            while let Some(ast) = semicolon.pop_front() {
                eval(ast, state)?;
            }
        }
        flat_ast::FlatAst::Pipe(mut pipe) => {
            state.pipe = Pipe::open();

            while let Some(command) = pipe.pop_front() {
                // let is_redirect = command.redirects.is_empty();

                let is_background = command.background;

                let mut ps_command = create_process_command(command, state)?;

                // if pipe.is_empty() && is_redirect{
                //     ps_command.stdout(process::Stdio::inherit());
                // }

                if pipe.is_empty() {
                    ps_command.stdout(process::Stdio::inherit());
                }

                let pid = state.handler.push(
                    ps_command.spawn().map_err(|err| {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            Error::new(ErrorKind::NotFound, "Command not found")
                        } else {
                            Error::new(ErrorKind::EngineError, "Command execution failed")
                        }
                    })?,
                    is_background,
                );

                if let Some(child) = state.handler.get(pid) {
                    if let Some(stdout) = child.stdout.as_ref() {
                        let fd = stdout.as_raw_fd();
                        state.pipe.send(fd);
                    }
                }
            }

            state.pipe.close();

            state.handler.wait();
        }

        flat_ast::FlatAst::Statement(statement) => match statement.to_owned() {
            flat_ast::Statement::Command(command) => {
                let is_background = command.background;

                let mut ps_command = create_process_command(command, state)?;

                state.handler.push(
                    ps_command.spawn().map_err(|err| {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            Error::new(ErrorKind::NotFound, "Command not found")
                        } else {
                            Error::new(ErrorKind::EngineError, "Command execution failed")
                        }
                    })?,
                    is_background,
                );

                state.handler.wait();
            }

            flat_ast::Statement::Assign(assign) => {
                let ident = match assign.ident {
                    flat_ast::Expr::Ident(ident) => ident,
                    _ => Err(Error::new(
                        ErrorKind::EngineError,
                        "Command execution failed. Invalid abstract syntax tree",
                    ))?,
                };

                let value = match assign.expr {
                    flat_ast::Expr::String(string) => string,

                    flat_ast::Expr::USize(number) => number.to_string(),

                    _ => Err(Error::new(
                        ErrorKind::EngineError,
                        "Command execution failed. Invalid abstract syntax tree",
                    ))?,
                };

                state.vars.insert(&ident, &value);
            }
        },
    }

    Ok(())
}

fn create_process_command(
    command: flat_ast::Command,
    state: &mut State,
) -> Result<process::Command> {
    // name
    let program = get_command_name(command.expr, state)?;

    // args
    let args = get_command_args(command.args, state)?;

    // stdin, stdout, stderr
    let (stdin, stdout, stderr) = set_command_stdio(state);

    let mut ps_command = process::Command::new(program);

    ps_command.args(args);

    ps_command.stdin(stdin);

    ps_command.stdout(stdout);

    ps_command.stderr(stderr);

    Ok(ps_command)
}

// Set command stdin, stdout, stderr with redirect
//
// If a pipe is attached to Stdin, Stdout, or Stderr, and any of them overlaps with Redirect, the pipe will be discarded.
//
// specifically, it closes the pipe in the State structure.
//
// fn set_command_stdio_redirect(
//     redirects: Vec<flat_ast::Redirect>,
//     stdin: process::Stdio,
//     stdout: process::Stdio,
//     stderr: process::Stdio,
//     state: &mut State,
// ) -> (process::Stdio, process::Stdio, process::Stdio) {
//     if redirects.is_empty() {
//         return (stdin, stdout, stderr);
//     }

//     redirects.iter().for_each(|redirect| todo!());

//     todo!()
// }

/// Set command stdin, stdout, stderr
fn set_command_stdio(state: &mut State) -> (process::Stdio, process::Stdio, process::Stdio) {
    // stdin
    let stdin = if state.pipe.is_recvable() {
        unsafe { process::Stdio::from_raw_fd(state.pipe.recv()) }
    } else {
        process::Stdio::inherit()
    };

    // stdout
    let stdout = if state.pipe.is_sendable() {
        process::Stdio::piped()
    } else {
        process::Stdio::inherit()
    };

    // stderr
    let stderr = process::Stdio::inherit();

    (stdin, stdout, stderr)
}

/// Get command arguments
fn get_command_args(args: Vec<flat_ast::Expr>, state: &mut State) -> Result<Vec<String>> {
    let mut v = Vec::with_capacity(args.len());

    args.iter().for_each(|arg| match arg.to_owned() {
        flat_ast::Expr::String(string) => v.push(string),
        flat_ast::Expr::Ident(ident) => v.push(
            state
                .vars
                .get(&ident)
                .unwrap_or(&String::default())
                .to_string(),
        ),
        flat_ast::Expr::USize(number) => v.push(number.to_string()),
        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Command execution failed. Invalid abstract syntax tree",
        ))
        .unwrap(),
    });

    Ok(v)
}

/// Get command name
fn get_command_name(expr: flat_ast::Expr, state: &mut State) -> Result<String> {
    match expr {
        flat_ast::Expr::String(string) => Ok(string),

        flat_ast::Expr::Ident(ident) => Ok(state
            .vars
            .get(&ident)
            .unwrap_or(&String::default())
            .to_string()),

        flat_ast::Expr::USize(number) => Ok(number.to_string()),

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Command execution failed. Invalid abstract syntax tree",
        )),
    }
}
