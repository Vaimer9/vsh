use std::env;
use std::env::args;
use std::io::{stdin, stdout, Result, Write};

use rustyline::error::ReadlineError;
use rustyline::Editor;
use ansi_term::{Colour, Style};

mod eval;

pub const shell_prompt: char = 'λ';

pub struct Prompt {
    character: char,
    is_init: bool
}

impl Prompt {

    pub fn new(character: char) -> Self {
        Self { character, is_init: false}
    }

    pub fn start_shell() -> io::Result<()> {
        let mut rl = Editor::<()>::new();

        if !self.is_init { self.init()? } else {
            if rl.load_history("history.txt").is_err() {
                eprintln!("No previous history.");
            }
        }

        loop {
            let current_dir = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()?;

            println!("{}", Colour::Red.bold().paint(current_dir));
            let readline = rl.readline(" {} ", self.charecter);
            
            match readline {
                Ok(x) => {
                    rl.add_history_entry(line.as_str());
                    eval::Internalcommand::new(x).eval()?;
                },
                Err(ReadlineError::Interrupted) => break, 
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
            rl.save_history("history.txt")?;

        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        if let Some(x) = ProjectDirs::from("vsh", "vsh", "vsh") {
            match Path::new(x.config_dir()).is_dir() {
                true => {
                    fs::create_dir_all(x.config_dir())?;
                    let mut file = File::create(x.config_dir().join("history.txt"))?;
                }
                false => {
                    self.is_init = true;
                }
            }
        }

        self.start_shell()?;
        Ok(())
    }
}
