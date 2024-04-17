use fsh_common::{Error, ErrorKind, Result};

use super::ShVars;

pub(super) fn extract_command_args(
    command: &fsh_ast::Command,
    sh_vars: &mut ShVars,
) -> Result<Vec<String>> {
    let mut v = Vec::with_capacity(command.args.len());

    for arg in command.args.to_owned() {
        let arg = match arg {
            fsh_ast::Expr::String(string) => {
                let mut string_vec = globbing(&string);

                if string_vec.len() > 0 {
                    v.append(&mut string_vec);

                    continue;
                } else {
                    string
                }
            }

            fsh_ast::Expr::Ident(ident) => sh_vars.get(&ident).unwrap_or_default().to_string(),

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
    sh_vars: &mut ShVars,
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

fn globbing(path: &str) -> Vec<String> {
    if path.is_empty() {
        return vec![path.to_string()];
    }

    let mut v = Vec::new();

    match glob::glob(&path) {
        Ok(paths) => {
            let paths = paths
                .map(|path| path.unwrap().to_str().unwrap().to_string())
                .collect::<Vec<String>>();

            if paths.len() > 0 {
                for path in paths {
                    v.push(path);
                }
            } else {
                return vec![path.to_string()];
            }
        }
        Err(_) => return vec![path.to_string()],
    };

    v
}
