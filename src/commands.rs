use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn cd(args: Vec<String>) {
    match args.iter().next() {
        Some(e) => match env::set_current_dir(Path::new(e)) {
            Ok(_) => (),
            Err(_) => eprintln!("No such directory"),
        },
        None => eprintln!("Please specify a directory"),
    }
}

pub fn cd_for_string(arg: String) {
    match env::set_current_dir(Path::new(&arg)) {
        Ok(_) => (),
        Err(_) => eprintln!("No such directory"),
    }
}

pub fn neutral(x: String, y: Vec<String>) {
    match Command::new(&x).args(y).spawn() {
        Ok(mut ok) => {
            ok.wait();
        }
        Err(_) => eprintln!("No such command as `{}`", x),
    }
}
