use std::env;
use std::fs::File;
use std::io;
use std::process;

use crate::eval::{CommandError, Internalcommand};
use crate::prompt::Prompt;

use colored::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    character: char,
}

impl Repl {
    pub fn new(character: char) -> Self {
        Self { character }
    }

    pub fn start_shell(&mut self) -> io::Result<()> {
        let mut rl = Editor::<()>::new();
        let home_dir = env::var("HOME").unwrap(); // There should be a HOME dir so no need to worry about this unwrap

        if rl
            .load_history(&format!("{}/.vsh_history", home_dir))
            .is_err()
        {
            eprintln!("No previous history.");
            File::create(format!("{}/.vsh_history", home_dir)).expect("Can't create history File!");
        }
        let prompt = Prompt::new().generate_prompt();

        loop {
            let readline = rl.readline(prompt.as_str());

            match readline {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());
                    if let Err(e) = Self::run(x) {
                        match e {
                            CommandError::Exit => {
                                rl.save_history(&format!("{}/.vsh_history", home_dir))
                                    .expect("Couldn't Save History");
                                process::exit(0);
                            }
                            _ => continue // TODO: What should happen if an error is returned?
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => println!(),
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
            rl.save_history(&format!("{}/.vsh_history", home_dir))
                .expect("Couldn't Save History");
        }
        Ok(())
    }

    pub fn run(x: String) -> Result<(), CommandError> {
        let mut last_return = Ok(());
        for com in x.split(";") {
            last_return = Self::run_linked_commands(com.into());
        }
        last_return
    }
    fn run_command(com: String) -> Result<(), CommandError> {
        Internalcommand::new(com.to_string()).eval()
    }
    fn run_linked_commands(commands: String) -> Result<(), CommandError> {
        for linked_com in commands.split("&&") {
            if let Err(e) = Self::run_command(linked_com.to_string()) {
                return Err(e);
            }
        }
        Ok(())
    }
}
