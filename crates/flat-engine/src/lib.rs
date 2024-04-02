mod pipe;
mod process_handler;
mod sh_vars;
use flat_common::{error::Error, result::Result};
pub use pipe::*;
pub use process_handler::*;
pub use sh_vars::*;

use std::{
    os::unix::io::{AsRawFd, FromRawFd},
    process,
};

/*
    ToDo: Error, Redirect
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
        Self { vars, handler, pipe }
    }
}

pub fn eval(ast: flat_ast::FlatAst, state: &mut State) -> Result<()> {
    match ast.to_owned() {
        flat_ast::FlatAst::Semicolon(mut semicolon) => {
            // Bad!
            semicolon.reverse();

            while let Some(ast) = semicolon.pop() {
                eval(ast, state)?;
            }
        }
        flat_ast::FlatAst::Pipe(mut pipe) => {
            // Bad!
            pipe.commands.reverse();

            state.pipe = Pipe::open();

            while let Some(command) = pipe.commands.pop() {
                let mut ps_command = create_process_command(command, state);

                if pipe.commands.is_empty() {
                    ps_command.stdout(process::Stdio::inherit());
                }

                let pid = state
                    .handler
                    .push(ps_command.spawn().expect("Failed to spawn process"));

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
