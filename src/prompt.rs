use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use crate::eval::Internalcommand;
use colored::*;
use directories::ProjectDirs;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Prompt {
    character: char,
    is_init: bool,
}

impl Prompt {
    pub fn new(character: char) -> Self {
        Self {
            character,
            is_init: false,
        }
    }

    pub fn start_shell(&mut self) -> io::Result<()> {
        let mut rl = Editor::<()>::new();

        //if !self.is_init { self.init()?; } else {
        //    if rl.load_history(&get_history_dir()).is_err() {
        //        eprintln!("No previous history.");
        //    }
        //}

        loop {
            let current_dir = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
            let dir_prompt = format!("   {}", current_dir);
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
                    Internalcommand::new(x).eval()?;
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
            // rl.save_history(&get_history_dir()).unwrap();
        }
        Ok(())
    }

    pub fn init(&mut self) -> io::Result<()> {
        if let Some(x) = ProjectDirs::from("vsh", "vsh", "vsh") {
            match Path::new(x.config_dir()).is_dir() {
                true => {
                    fs::create_dir_all(x.config_dir())?;
                    let _file = File::create(x.config_dir().join("history.txt"))?;
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

pub fn get_history_dir() -> PathBuf {
    let mut p = PathBuf::new();
    if let Some(x) = ProjectDirs::from("vsh", "vsh", "vsh") {
        p = x.config_dir().join("history.txt");
    }
    p
}
