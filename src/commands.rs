use std::env;
use std::path::Path;
use std::process::Command;

use crate::eval::CommandError;

pub fn cd(arg: Option<&String>) {
    match arg {
        Some(dir ) => {
            match env::set_current_dir(Path::new(dir)) {
                Ok(_) => (),
                Err(_) => eprintln!("No such directory"),
            }
        }
        None => env::set_current_dir(env::var("HOME").unwrap()).expect("Could not go to home directory!"), // HOME will always be set
    }
}

pub fn neutral(x: String, y: Vec<String>) -> Result<(), CommandError> {
    match Command::new(&x).args(y).spawn() {
        Ok(mut ok) => {
            if let Ok(status) = ok.wait() {
                match status.code() {
                    Some(code) => {
                        if code > 0 {
                            Err(CommandError::Finished(code))
                        } else {
                            Ok(())
                        }
                    },
                    // This is only returned if the process was terminated by user
                    None => Err(CommandError::Terminated(127)), // Unfortunately getting the signal is still in nightly
                }
            } else {
                // This is the case in which the command could not be run
                eprintln!("Command could not be executed");
                Err(CommandError::Error)
            }
        }
        Err(_) => {
            eprintln!("No such command as `{}`", x);
            Err(CommandError::Error)
        }
    }
}
