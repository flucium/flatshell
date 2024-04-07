mod pipe;
mod process_handler;
mod sh_vars;
use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
pub use pipe::*;
pub use process_handler::*;
pub use sh_vars::*;

use std::{
    os::unix::io::{AsRawFd, FromRawFd},
    process,
};

/*
    ToDo: Redirect, Builtin Command
*/

#[derive(Debug)]
pub struct State {
    vars: ShVars,
    handler: ProcessHandler,
    pipe: Pipe,
}

impl State {
    pub fn new() -> Self {
        Self {
            vars: ShVars::new(),
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
        }
    }

    pub fn vars(&mut self) -> &mut ShVars {
        &mut self.vars
    }
}

impl From<ShVars> for State {
    fn from(vars: ShVars) -> Self {
        Self {
            vars,
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
        }
    }
}

impl From<ProcessHandler> for State {
    fn from(handler: ProcessHandler) -> Self {
        Self {
            vars: ShVars::new(),
            handler,
            pipe: Pipe::new(),
        }
    }
}

impl From<Pipe> for State {
    fn from(pipe: Pipe) -> Self {
        Self {
            vars: ShVars::new(),
            handler: ProcessHandler::new(),
            pipe,
        }
    }
}

impl From<(ShVars, ProcessHandler)> for State {
    fn from((vars, handler): (ShVars, ProcessHandler)) -> Self {
        Self {
            vars,
            handler,
            pipe: Pipe::new(),
        }
    }
}

impl From<(ShVars, Pipe)> for State {
    fn from((vars, pipe): (ShVars, Pipe)) -> Self {
        Self {
            vars,
            handler: ProcessHandler::new(),
            pipe,
        }
    }
}

impl From<(ProcessHandler, Pipe)> for State {
    fn from((handler, pipe): (ProcessHandler, Pipe)) -> Self {
        Self {
            vars: ShVars::new(),
            handler,
            pipe,
        }
    }
}

impl From<(ShVars, ProcessHandler, Pipe)> for State {
    fn from((vars, handler, pipe): (ShVars, ProcessHandler, Pipe)) -> Self {
        Self {
            vars,
            handler,
            pipe,
        }
    }
}

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

                let mut ps_command = create_process_command(command, state);

                // if pipe.is_empty() && is_redirect{
                //     ps_command.stdout(process::Stdio::inherit());
                // }

                if pipe.is_empty() {
                    ps_command.stdout(process::Stdio::inherit());
                }

                let pid = state.handler.push(
                    ps_command
                        .spawn()
                        .map_err(|_| Error::new(ErrorKind::NotFound, ""))?,
                );

                if let Some(child) = state.handler.get(pid) {
                    if let Some(stdout) = child.stdout.as_ref() {
                        state.pipe.send(stdout.as_raw_fd());
                    }
                }
            }

            state.pipe.close();

            state.handler.wait();
        }

        flat_ast::FlatAst::Statement(statement) => match statement.to_owned() {
            flat_ast::Statement::Command(command) => {
                let mut ps_command = create_process_command(command, state);

                state
                    .handler
                    .push(ps_command.spawn().expect("Failed to spawn process"));

                state.handler.wait();
            }

            flat_ast::Statement::Assign(assign) => {
                let ident = match assign.ident {
                    flat_ast::Expr::Ident(ident) => ident,
                    _ => Err(Error::DUMMY)?,
                };

                let value = match assign.expr {
                    flat_ast::Expr::String(string) => string,

                    flat_ast::Expr::USize(number) => number.to_string(),

                    _ => Err(Error::DUMMY)?,
                };

                state.vars.insert(&ident, &value);
            }
        },
    }

    Ok(())
}

fn create_process_command(command: flat_ast::Command, state: &mut State) -> process::Command {
    // name
    let program = get_command_name(command.expr, state);

    // args
    let args = get_command_args(command.args, state);

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

    // redirect
    // command.redirects.iter().for_each(|redirect| {
    //     let left = match redirect.left {
    //         flat_ast::Expr::FD(fd) => fd,
    //         _ => {
    //             todo!()
    //         }
    //     };

    //     let right = match redirect.right.to_owned() {
    //         flat_ast::Expr::String(string) => fs::File::options().create(true).write(true).read(true).open(string).unwrap(),
    //         flat_ast::Expr::FD(fd) => unsafe { fs::File::from_raw_fd(fd) },
    //         _ => {
    //             todo!()
    //         }
    //     };

    //     if !matches!(left, 0 | 1 | 2) { /*dup2 */ }

    //     match redirect.operator {
    //         flat_ast::RedirectOperator::Gt => {
    //             if left == 1 {
    //                 stdout = process::Stdio::from(right);
    //             } else if left == 2 {
    //                 stderr = process::Stdio::from(right);
    //             } else {
    //                 // error
    //             }
    //         }
    //         flat_ast::RedirectOperator::Lt => {
    //             if left == 0 {
    //                 stdin = process::Stdio::from(right);
    //             } else {
    //                 // error
    //             }
    //         }
    //     }
    // });

    let mut ps_command = process::Command::new(program);

    ps_command.args(args);

    ps_command.stdin(stdin);

    ps_command.stdout(stdout);

    ps_command.stderr(stderr);

    ps_command
}

fn get_command_args(args: Vec<flat_ast::Expr>, state: &mut State) -> Vec<String> {
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
        _ => {
            todo!()
        }
    });

    v
}

fn get_command_name(expr: flat_ast::Expr, state: &mut State) -> String {
    match expr {
        flat_ast::Expr::String(string) => string,

        flat_ast::Expr::Ident(ident) => state
            .vars
            .get(&ident)
            .unwrap_or(&String::default())
            .to_string(),

        flat_ast::Expr::USize(number) => number.to_string(),

        _ => todo!(),
    }
}
