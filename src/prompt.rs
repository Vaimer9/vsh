use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

use crate::eval::{CommandError, Internalcommand};
use colored::*;
use directories::ProjectDirs;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Prompt {
    character: char,
}

impl Prompt {
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

        loop {
            let current_dir = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
            let dir_prompt = format!("   {} ", current_dir);
            let shell_char = format!("{}", self.character);

            println!(
                "{}{}{}",
                "".truecolor(109, 152, 134),
                dir_prompt
                    .on_truecolor(109, 152, 134)
                    .truecolor(33, 33, 33)
                    .bold(),
                "".truecolor(109, 152, 134)
            );
            let readline = rl.readline(format!("{} ", shell_char.green()).as_str());

            match readline {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());
                    match Self::run(x) {
                        CommandError::Exit => {
                            rl.save_history(&format!("{}/.vsh_history", home_dir))
                                .expect("Couldn't Save History");
                            process::exit(0);
                        }
                        CommandError::Ok | CommandError::Error => (),
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

    pub fn run(x: String) -> CommandError {
        let mut returner = CommandError::Ok;

        // TODO: Refactor this wall of match :[
        match x.split_once(";") {
            None => match x.split_once("&&") {
                None => return Internalcommand::new(x).eval(),
                Some((a, b)) => match Internalcommand::new(a.to_string()).eval() {
                    CommandError::Error => returner = CommandError::Ok,
                    CommandError::Ok => match Internalcommand::new(b.to_string()).eval() {
                        CommandError::Ok | CommandError::Error => returner = CommandError::Ok,
                        CommandError::Exit => returner = CommandError::Exit,
                    },
                    CommandError::Exit => returner = CommandError::Exit,
                },
            },
            Some((a, b)) => {
                match Internalcommand::new(a.to_string()).eval() {
                    CommandError::Ok | CommandError::Error => returner = CommandError::Ok,
                    CommandError::Exit => returner = CommandError::Exit,
                }

                match Internalcommand::new(b.to_string()).eval() {
                    CommandError::Ok | CommandError::Error => returner = CommandError::Ok,
                    CommandError::Exit => returner = CommandError::Exit,
                }
            }
        }
        returner
    }
}
