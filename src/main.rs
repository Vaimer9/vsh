mod prompt;
mod eval;

use prompt::Prompt;


const shell_char: char = 'λ';

fn main() {
    Prompt::new(shell_char).start_shell().unwrap();
}
