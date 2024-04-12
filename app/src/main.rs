use flat_engine::{eval, State};
use flat_parser::Parser;
use flat_terminal::Terminal;

/*
    ToDo: Error handling
*/

/*
    PROFILE: $HOME/.fsh_profile

    DEBUG PROFILE: ./temp/.fsh_profile
*/

/*
    $FSH_PROMPT = "fsh> "
    $FSH_HISTORY = ".fsh_history"
    $FSH_HISTORY_SIZE = 1000
    $FSH_HISTORY_ENABLED = true
*/

fn main() {
    let mut state = State::new();

    let mut terminal = Terminal::new();

    loop {
        terminal.set_prompt(flat_terminal::prompt("\\W> "));

        let string = terminal.read_line().unwrap();

        let ast = Parser::new(&string).parse().unwrap();

        eval(ast, &mut state).unwrap();
    }
}
