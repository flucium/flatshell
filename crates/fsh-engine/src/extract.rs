use fsh_common::{Error,ErrorKind,Result};

use super::ShVars;

pub(super) fn extract_command_args(
    command: &fsh_ast::Command,
    sh_vars: &mut ShVars
) -> Result<Vec<String>> {
    let mut v = Vec::with_capacity(command.args.len());

    for arg in command.args.to_owned() {
        let arg = match arg {
            fsh_ast::Expr::String(string) => string,

            fsh_ast::Expr::Ident(ident) => {
                sh_vars.get(&ident).unwrap_or_default().to_string()
            }

            fsh_ast::Expr::Number(number) => number.to_string(),

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
    command: &fsh_ast::Command,
    sh_vars: &mut ShVars
) -> Result<String> {
    let name = match command.expr.to_owned() {
        fsh_ast::Expr::String(string) => string,

        fsh_ast::Expr::Ident(ident) => sh_vars.get(&ident).unwrap_or_default().to_string(),

        fsh_ast::Expr::Number(number) => number.to_string(),

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Extract command name error: Invalid abstract syntax tree",
        ))?,
    };

    Ok(name)
}

pub(super) fn extract_assign(assign: fsh_ast::Assign) -> Result<(String, String)> {
    let key = match assign.ident {
        fsh_ast::Expr::Ident(key) => key,

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Extract assign error: Invalid abstract syntax tree",
        ))?,
    };

    let value = match assign.expr {
        fsh_ast::Expr::String(value) => value,

        _ => Err(Error::new(
            ErrorKind::EngineError,
            "Extract assign error: Invalid abstract syntax tree",
        ))?,
    };

    Ok((key, value))
}