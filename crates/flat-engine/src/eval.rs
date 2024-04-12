use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};

use std::{
    fs,
    os::{
        fd::IntoRawFd,
        unix::{
            io::{AsRawFd, FromRawFd},
            process::CommandExt,
        },
    },
    process,
};

use super::{extract::*, Pipe, State};

pub fn eval(ast: flat_ast::FlatAst, state: &mut State) -> Result<()> {
    match ast {
        flat_ast::FlatAst::Semicolon(mut semicolon) => {
            while let Some(ast) = semicolon.pop_front() {
                eval(ast, state)?;
            }
        }
        flat_ast::FlatAst::Pipe(mut pipe) => {
            *state.pipe_mut() = Pipe::open();

            while let Some(command) = pipe.pop_front() {
                eval_command(command, state, pipe.is_empty())?;
            }

            state.pipe_mut().close()?;

            state.handler_mut().wait();
        }
        flat_ast::FlatAst::Statement(statement) => match statement {
            flat_ast::Statement::Command(command) => {
                eval_command(command, state, true)?;

                state.handler_mut().wait();
            }
            flat_ast::Statement::Assign(assign) => {
                eval_assign(assign, state)?;
            }
        },
    }

    Ok(())
}

fn eval_command(command: flat_ast::Command, state: &mut State, is_last: bool) -> Result<()> {
    let name = extract_command_name(&command, state)?;

    let args = extract_command_args(&command, state)?;

    let redirects = command.redirects;

    eval_builtin_command(&name, &args, state).or_else(|_| {
        eval_process_command(name, args, redirects, command.background, state, is_last)
    })?;

    Ok(())
}

fn eval_builtin_command(name: &String, args: &Vec<String>, state: &mut State) -> Result<()> {
    match name.as_str() {
        "exit" => {
            let code = args
                .get(0)
                .unwrap_or(&"0".to_string())
                .parse::<i32>()
                .unwrap_or(2);
            flat_builtin::common::exit(code);
        }
        "abort" => {
            flat_builtin::common::abort();
        }
        "printenv" => {
            let s = &"".to_string();

            let key = args.get(0).unwrap_or(s);

            flat_builtin::common::printenv(key, state.vars().entries());
        }
        _ => Err(Error::DUMMY)?,
    }

    Ok(())
}

fn eval_process_command(
    name: String,
    args: Vec<String>,
    redirects: Vec<flat_ast::Redirect>,
    is_background: bool,
    state: &mut State,
    is_last: bool,
) -> Result<()> {
    // create a new process command
    let mut ps_command = process::Command::new(&name);

    // set the arguments
    ps_command.args(args);

    // set the stdin, stdout, and stderr
    let (stdin, stdout, stderr) = set_command_stdio(state);

    ps_command.stdin(stdin).stdout(stdout).stderr(stderr);

    if is_last {
        ps_command.stdout(process::Stdio::inherit());
    }

    // set the pre-execution closure
    unsafe {
        ps_command.pre_exec(move || {
            for redirect in redirects.to_owned() {
                let left = match redirect.left.to_owned() {
                    flat_ast::Expr::FD(fd) => fd,
                    _ => {
                        todo!()
                    }
                };

                let right = match redirect.right.to_owned() {
                    flat_ast::Expr::String(string) => fs::File::options()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(string)
                        .unwrap()
                        .into_raw_fd(),

                    flat_ast::Expr::Ident(_) => {
                        todo!()
                    }
                    flat_ast::Expr::USize(number) => fs::File::options()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(number.to_string())
                        .unwrap()
                        .into_raw_fd(),
                    flat_ast::Expr::FD(fd) => fd,
                };

                match redirect.operator {
                    flat_ast::RedirectOperator::Gt => {
                        libc::dup2(right, left);
                    }
                    flat_ast::RedirectOperator::Lt => {
                        libc::dup2(right, left);
                    }
                }
            }

            Ok(())
        });
    };

    // spawn the process
    let child = ps_command.spawn().map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            Error::new(ErrorKind::NotFound, "Command not found")
        } else {
            Error::new(ErrorKind::EngineError, "Command execution failed")
        }
    })?;

    // push the process to the handler
    let pid = state.handler_mut().push(child, is_background);

    // send the stdout to the pipe
    if let Some(child) = state.handler().get(pid) {
        if let Some(stdout) = child.stdout.as_ref() {
            let fd = stdout.as_raw_fd();
            state.pipe_mut().send(fd).unwrap();
        }
    }

    Ok(())
}

fn set_command_stdio(state: &mut State) -> (process::Stdio, process::Stdio, process::Stdio) {
    // stdin
    let stdin = if state.pipe().is_recvable() {
        unsafe { process::Stdio::from_raw_fd(state.pipe_mut().recv().unwrap()) }
    } else {
        process::Stdio::inherit()
    };

    // stdout
    let stdout = if state.pipe().is_sendable() {
        process::Stdio::piped()
    } else {
        process::Stdio::inherit()
    };

    // stderr
    let stderr = process::Stdio::inherit();

    (stdin, stdout, stderr)
}

fn eval_assign(assign: flat_ast::Assign, state: &mut State) -> Result<()> {
    let (key, value) = extract_assign(assign)?;

    state.vars_mut().insert(key, value);

    Ok(())
}
