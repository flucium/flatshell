use flat_common::error::{Error, ErrorKind};
use flat_common::result::Result;

use super::State;

pub(super) fn extract_command_args(
    command: &flat_ast::Command,
    state: &mut State,
) -> Result<Vec<String>> {
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

pub(super) fn extract_command_name(
    command: &flat_ast::Command,
    state: &mut State,
) -> Result<String> {
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

pub(super) fn extract_assign(assign: flat_ast::Assign) -> Result<(String, String)> {
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
