use std::env;
use std::path::Path;

use crate::eval::CommandError;

use crate::command::{CommandStructure, expand};

pub struct Cd;

impl CommandStructure for Cd {
    fn name() -> &'static str {
        "cd"
    }

    fn about() -> &'static str {
        "A command line program for changing working directory"
    }

    fn examples() -> [&'static str; 3] {
        [
            "cd",
            "cd ~/Downloads",
            "cd .."
        ]
    }

    fn run(args: Vec<String>) -> Result<(), CommandError> {
        match args.get(0) {
            Some(dir) => {
                if env::set_current_dir(Path::new(&expand(dir.to_string()))).is_err() {
                    Err(CommandError::Error("No such directory".to_string()))
                } else { Ok(()) }
            }
            None => {
                if env::set_current_dir(env::var("HOME").unwrap()).is_err() {
                    Err(CommandError::Error("Could not enter HOME directory".to_string()))
                } else { Ok(()) }
            }
        }
    }
}
