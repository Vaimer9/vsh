use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::eval::CommandError;

pub fn cd(arg: &String) {
    match env::set_current_dir(Path::new(&arg)) {
        Ok(_) => (),
        Err(_) => eprintln!("No such directory"),
    }
}

pub fn neutral(x: String, y: Vec<String>) -> CommandError {
    match Command::new(&x).args(y).spawn() {
        Ok(mut ok) => {
            ok.wait().unwrap();
            return CommandError::Ok;
        }
        Err(_) => {
            eprintln!("No such command as `{}`", x);
            return CommandError::Error;
        }
    }
}
