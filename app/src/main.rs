use std::fs;

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
    $FSH_PATH = "/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin"
*/

struct Shell {
    state: State,
    terminal: Terminal,
}

impl Shell {
    fn new() -> Self {
        Shell {
            state: State::new(),
            terminal: Terminal::new(),
        }
    }

    fn open_profile(&mut self) {
        let profile = fs::read_to_string("./temp/profile.fsh").unwrap();

        let ast = Parser::new(&profile).parse().unwrap();

        eval(ast, &mut self.state).unwrap();

        self.state.vars_mut().inherit(std::env::vars());
    }

    fn repl(&mut self) {
        loop {
            self.terminal.set_prompt(flat_terminal::prompt(
                self.state.vars().get("FSH_PROMPT").unwrap_or_default(),
            ));

            let string = self.terminal.read_line().unwrap();

            let ast = Parser::new(&string).parse().unwrap();

            eval(ast, &mut self.state).unwrap();
        }
    }
}

fn main() {
    let mut shell = Shell::new();

    shell.open_profile();

    shell.repl();
}
