mod eval;
mod prompt;
mod repl;
mod utils;

use repl::Repl;

fn main() {
    std::process::exit(Repl::new().start_shell().unwrap_or(1));
}
