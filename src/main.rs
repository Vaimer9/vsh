mod commands;
mod eval;
mod prompt;
mod repl;
mod utils;

use repl::Repl;

const SHELL_CHAR: char = 'λ';

fn main() {
    Repl::new(SHELL_CHAR).start_shell().unwrap();
}
