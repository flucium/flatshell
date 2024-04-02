fn main() {
    // REP LOOP. Demo.
    let mut terminal = flat_terminal::Terminal::new();

    let mut state = flat_engine::State::new();

    // ping -c 3 127.0.0.1 | cat -b
    // $A = ping; $A -c 3 127.0.0.1 | cat -b
    terminal.set_prompt("> ");
    loop {
        let input = terminal.read_line().unwrap();

        let ast = flat_parser::Parser::new(flat_parser::Lexer::new(&input))
            .parse()
            .unwrap();

        flat_engine::eval(ast, &mut state).unwrap();
    }
}
