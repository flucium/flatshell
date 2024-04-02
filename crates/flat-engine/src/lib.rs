mod pipe;
mod process_handler;
mod sh_vars;
use flat_common::{error::Error, result::Result};
// pub use pipe::*;
pub use process_handler::*;
pub use sh_vars::*;
use std::process;
/*
    ToDo: Pipe, Error, etc...
*/
pub struct State {
    vars: ShVars,
    handler: ProcessHandler,

    /*
        Bad codes
    */
    stdin: Option<process::Stdio>,
    stdout: Option<process::Stdio>,
    stderr: Option<process::Stdio>,
}

impl State {
    pub fn new() -> Self {
        Self {
            vars: ShVars::new(),
            handler: ProcessHandler::new(),
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}

impl From<ShVars> for State {
    fn from(vars: ShVars) -> Self {
        Self {
            vars,
            handler: ProcessHandler::new(),
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}

impl From<ProcessHandler> for State {
    fn from(handler: ProcessHandler) -> Self {
        Self {
            vars: ShVars::new(),
            handler,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}

impl From<(ShVars, ProcessHandler)> for State {
    fn from((vars, handler): (ShVars, ProcessHandler)) -> Self {
        Self {
            vars,
            handler,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}

pub fn eval(ast: flat_ast::FlatAst, state: &mut State) -> Result<()> {
    match ast.to_owned() {
        flat_ast::FlatAst::Semicolon(mut semicolon) => {

            // Bad code
            semicolon.reverse();

            while let Some(ast) = semicolon.pop() {
                eval(ast, state)?;
            }
        }

        flat_ast::FlatAst::Pipe(mut pipe) => {
            // Bad code
            pipe.commands.reverse();

            // Bad code
            state.stdout = Some(process::Stdio::piped());

            while let Some(command) = pipe.commands.pop() {
                let mut ps_command = create_process_command(command, state)?;

                // Bad code
                if pipe.commands.is_empty() {
                    ps_command.stdout(process::Stdio::inherit());
                } else {
                    ps_command.stdout(process::Stdio::piped());
                }

                // Bad codes
                if let Some(child) = ps_command.spawn().ok() {
                    let pid = state.handler.push(child);

                    if let Some(child) = state.handler.get_mut(pid) {
                        if let Some(stdout) = child.stdout.take() {
                            state.stdin = Some(process::Stdio::from(stdout));
                        }
                    }
                }
            }

            state.handler.wait();
        }

        flat_ast::FlatAst::Statement(statement) => match statement {
            flat_ast::Statement::Command(command) => {
                
                let mut ps_command = create_process_command(command, state)?;

                state
                    .handler
                    .push(ps_command.spawn().map_err(|_| Error::DUMMY)?);

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
                
                state.vars.set(&ident, &value);
                
            }
        },
    }

    Ok(())
}

fn create_process_command(
    command: flat_ast::Command,
    state: &mut State,
) -> Result<process::Command> {
    let program = get_command_name(command.expr, state)?;

    let args = get_command_args(command.args, state)?;

    let mut ps_command = process::Command::new(&program);

    ps_command.args(args);

    ps_command.stdin(state.stdin.take().unwrap_or(process::Stdio::inherit()));

    ps_command.stdout(state.stdout.take().unwrap_or(process::Stdio::inherit()));

    ps_command.stderr(state.stderr.take().unwrap_or(process::Stdio::inherit()));

    Ok(ps_command)
}

fn get_command_args(args: Vec<flat_ast::Expr>, state: &mut State) -> Result<Vec<String>> {
    let mut v = Vec::with_capacity(args.len());

    args.iter().for_each(|arg| match arg.to_owned() {
        flat_ast::Expr::String(string) => v.push(string),
        flat_ast::Expr::Ident(ident) => v.push(
            state
                .vars
                .get(&ident)
                .unwrap_or(&String::default())
                .to_owned(),
        ),
        flat_ast::Expr::USize(number) => v.push(number.to_string()),
        _ => {
            todo!()
        }
    });

    Ok(v)
}

fn get_command_name(expr: flat_ast::Expr, state: &mut State) -> Result<String> {

    match expr {
        flat_ast::Expr::String(string) => Ok(string),

        flat_ast::Expr::Ident(ident) => Ok(state
            .vars
            .get(&ident)
            .unwrap_or(&String::default())
            .to_owned()),

        flat_ast::Expr::USize(number) => Ok(number.to_string()),

        _ => Err(Error::DUMMY),
    }
}
