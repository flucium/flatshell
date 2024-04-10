mod pipe;
mod process_handler;
mod sh_vars;
mod state;

// pub use
pub use pipe::*;
pub use process_handler::*;
pub use sh_vars::*;
pub use state::*;

// use
use flat_common::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::{
    os::{fd::FromRawFd, unix::io::AsRawFd},
    process,
};

pub fn eval(ast: flat_ast::FlatAst, state: &mut State) -> Result<()> {
    match ast {
        flat_ast::FlatAst::Semicolon(mut semicolon) => {
            while let Some(ast) = semicolon.pop_front() {
                eval(ast, state)?;
            }
        }
        flat_ast::FlatAst::Pipe(mut pipe) => {}
        flat_ast::FlatAst::Statement(statement) => {
            match statement {
                // Command
                flat_ast::Statement::Command(command) => {
                    let mut ps_command = extract_command(&command, state)?;

                    state.handler_mut().push(
                        ps_command.spawn().map_err(|err| {
                            if err.kind() == std::io::ErrorKind::NotFound {
                                Error::new(ErrorKind::NotFound, "Command not found")
                            } else {
                                Error::new(ErrorKind::EngineError, "Command execution failed")
                            }
                        })?,
                        command.background,
                    );

                    state.handler_mut().wait();
                }

                // Assign
                flat_ast::Statement::Assign(assign) => {
                    let (key, value) = extract_assign(assign)?;
                    state.vars_mut().insert(key, value);
                }
            }
        }
    }
    Ok(())
}

fn extract_command(command: &flat_ast::Command, state: &mut State) -> Result<process::Command> {
    let name = extract_command_name(&command, state)?;

    let args = extract_command_args(&command, state)?;

    let mut ps_command = process::Command::new(name);

    ps_command
        .args(args)
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .envs(state.vars().entries());
    
    Ok(ps_command)
}

fn extract_command_args(command: &flat_ast::Command, state: &mut State) -> Result<Vec<String>> {
    let mut v = Vec::with_capacity(command.args.len());

    for arg in command.args.to_owned() {
        let arg = match arg {
            flat_ast::Expr::String(string) => string,

            flat_ast::Expr::Ident(ident) => {
                state.vars().get(&ident).unwrap_or_default().to_string()
            }

            flat_ast::Expr::USize(number) => number.to_string(),

            _ => Err(Error::new(
                ErrorKind::EngineError,
                "Extract command args error: Invalid abstract syntax tree",
            ))?,
        };
        v.push(arg);
    }

    Ok(v)
}

fn extract_command_name(command: &flat_ast::Command, state: &mut State) -> Result<String> {
    let name = match command.expr.to_owned() {
        flat_ast::Expr::String(string) => string,

        flat_ast::Expr::Ident(ident) => state.vars().get(&ident).unwrap_or_default().to_string(),

        flat_ast::Expr::USize(number) => number.to_string(),

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Extract command name error: Invalid abstract syntax tree",
        ))?,
    };

    Ok(name)
}

fn extract_assign(assign: flat_ast::Assign) -> Result<(String, String)> {
    let key = match assign.ident {
        flat_ast::Expr::Ident(key) => key,

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Extract assign error: Invalid abstract syntax tree",
        ))?,
    };

    let value = match assign.expr {
        flat_ast::Expr::String(value) => value,

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Extract assign error: Invalid abstract syntax tree",
        ))?,
    };

    Ok((key, value))
}
