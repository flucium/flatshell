use flat_engine::{eval, State};
use flat_parser::{Lexer, Parser};
use flat_terminal::{History, Terminal};

#[inline]
fn repl() {
    let state = &mut State::new();

    let mut terminal = Terminal::new();

    terminal.set_history(History::new());

    terminal.set_prompt("> ");

    loop {
        match terminal.read_line() {
            Err(e) => {
                panic!("Error: {}", e.message());
            }
            Ok(line) => {
                let ast = match Parser::new(Lexer::new(&line)).parse() {
                    Err(e) => {
                        panic!("Error: {}", e.message());
                    }
                    Ok(ast) => ast,
                };

                if let Err(err) = eval(ast, state) {
                    panic!("{}", err.message());
                }
            }
        }
    }
}

fn main() {
    repl();
}
