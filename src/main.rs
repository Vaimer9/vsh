mod commands;
mod eval;
mod prompt;
mod repl;
mod utils;

use repl::Repl;

fn main() {
    Repl::new().start_shell().unwrap();
}
