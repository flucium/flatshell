use fsh_engine::{eval, State};
use fsh_parser::Parser;
use fsh_terminal::{prompt, Terminal};

#[inline]
fn rep() {
    let mut state = State::new();
    state.vars_mut().inherit(std::env::vars());

    // read
    let mut terminal = Terminal::new();

    terminal.set_prompt(
        prompt(state
            .vars()
            .get("FSH_PROMPT")
            .unwrap_or("\\W$ ")
            .to_string()),
    );

    let line = terminal.read_line().unwrap_or_default();

    // eval and print
    let ast = match Parser::new(&line).parse() {
        Ok(ast) => ast,
        Err(e) => {
            todo!()
        }
    };

    if let Err(err) = eval(ast, &mut state) {
        todo!()
    }
}

fn repl() {
    loop {
        rep();
    }
}

fn main() {
    repl();
}
