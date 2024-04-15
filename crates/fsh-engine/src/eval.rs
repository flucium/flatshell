use fsh_common::{Error, ErrorKind, Result};

use std::{
    fs,
    os::{
        fd::IntoRawFd,
        unix::{
            io::{AsRawFd, FromRawFd},
            process::CommandExt,
        },
    },
    path::Path,
    process,
};

use super::{extract::*, pipe::Pipe, ShVars, State};

pub fn eval(ast: fsh_ast::Ast, state: &mut State, sh_vars: &mut ShVars) -> Result<()> {
    match ast {
        //
        //
        //
        fsh_ast::Ast::Semicolon(mut semicolon) => {
            while let Some(ast) = semicolon.pop_front() {
                eval(ast, state, sh_vars)?;
            }
        }

        //
        //
        //
        fsh_ast::Ast::Pipe(mut pipe) => {
            *state.pipe_mut() = Pipe::open();

            while let Some(command) = pipe.pop_front() {
                eval_command(command, state, sh_vars, pipe.is_empty())?;
            }

            state.pipe_mut().close()?;

            state.handler_mut().wait();
        }

        //
        //
        //
        fsh_ast::Ast::Statement(statement) => match statement {
            //
            //
            //
            fsh_ast::Statement::Command(command) => {
                eval_command(command, state, sh_vars, true)?;

                state.handler_mut().wait();
            }

            //
            //
            //
            fsh_ast::Statement::Assign(assign) => {
                eval_assign(assign, sh_vars)?;
            }
        },
    }

    Ok(())
}

fn eval_command(
    command: fsh_ast::Command,
    state: &mut State,
    sh_vars: &mut ShVars,
    is_last: bool,
) -> Result<()> {
    let name = extract_command_name(&command, sh_vars)?;

    let args = extract_command_args(&command, sh_vars)?;

    let redirects = command.redirects;

    eval_builtin_command(&name, &args, state).or_else(|_| {
        eval_process_command(
            name,
            args,
            redirects,
            command.background,
            state,
            sh_vars,
            is_last,
        )
    })?;

    Ok(())
}

//
//
//
fn eval_builtin_command(name: &String, args: &Vec<String>, state: &mut State) -> Result<()> {
    match name.as_ref() {
        //
        // Unix builtins
        //
        "cd" => {
            let arg = match args.first() {
                Some(arg) => arg,
                None => "/",
            };

            super::builtin::unix::cd(arg, state)?;
        }

        //
        // Common builtins
        //
        "abort" => {
            super::builtin::common::abort();
        }

        "exit" => {
            let code = match args.first() {
                Some(arg) => arg.parse().unwrap_or(0),
                None => 0,
            };

            super::builtin::common::exit(code);
        }

        _ => Err(Error::INTERNAL)?,
    }

    Ok(())
}

//
//
//
fn eval_process_command(
    name: String,
    args: Vec<String>,
    redirects: Vec<fsh_ast::Redirect>,
    is_background: bool,
    state: &mut State,
    sh_vars: &mut ShVars,
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

    // set the environment variables
    ps_command.envs(sh_vars.entries());

    // set the current directory
    ps_command.current_dir(state.current_dir().unwrap_or(Path::new("/")));

    // set the pre-execution closure
    unsafe {
        ps_command.pre_exec(move || {
            for redirect in redirects.to_owned() {
                let left = match redirect.left.to_owned() {
                    fsh_ast::Expr::FD(fd) => fd,
                    _ => {
                        todo!()
                    }
                };

                let right = match redirect.right.to_owned() {
                    fsh_ast::Expr::String(string) => fs::File::options()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(string)
                        .unwrap()
                        .into_raw_fd(),

                    fsh_ast::Expr::Ident(_) => {
                        todo!()
                    }
                    fsh_ast::Expr::Number(number) => fs::File::options()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(number.to_string())
                        .unwrap()
                        .into_raw_fd(),
                    fsh_ast::Expr::FD(fd) => fd,
                };

                match redirect.operator {
                    fsh_ast::RedirectOperator::Gt => {
                        libc::dup2(right, left);
                    }
                    fsh_ast::RedirectOperator::Lt => {
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

fn eval_assign(assign: fsh_ast::Assign, sh_vars: &mut ShVars) -> Result<()> {
    let (key, value) = extract_assign(assign)?;

    sh_vars.insert(key, value);

    Ok(())
}
