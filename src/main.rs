mod prompt;
mod eval;

use prompt::Prompt;
use std::io;

const shell_char: char = 'λ';

fn main() {
    Prompt::new(shell_char).start_shell().unwrap();
}
