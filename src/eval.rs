use std::env;
use std::env::args;
use std::io::{stdin, stdout, Result, Write};
use std::path::Path;
use std::process::Command;
use std::string::ToString;


pub struct Internalcommand {
    keyword: String,
    args: Vec<String>
}

impl Internalcommand {
    pub fn new(input: String) -> Self {
        let mut splitted = input.trim().split_whitespace();
        let keyword = match splitted.next() {
                some(x) => x.to_string(),
                none => String::from("") 
        };
        Self {
             keyword,
             args: splitted.map(ToString::to_string).collect::<Vec<String>>()
        }
    }

    pub fn eval(&mut self) -> io::Result<()> {
        match self {
            Self { keyword, args } => {
                match Command::new(keyword).args(args).spawn() {
                    Ok(mut ok) => ok.wait(),
                    Err(_) => eprintln!("No such command as `{}`", keyword)
                }
            },

            Self { keyword.as_str(): "cd", args } => {
                match args.iter().next() {
                    Some(e) => {
                        let path = Path::new(e);
                        match env::set_current_dir(path) {
                            Ok(_) => (),
                            Err(_) => eprintln!("No such directory")
                        }
                    }
                    None => eprintln!("Please specify a directory")
                }
            },

            Self { keyword.as_str(): "", .. } => {
                println!();
            },

            Self { keyword.as_str(): "exit", .. } => {
                process::exit(0);
            }
        }
        Ok(())
    }
}
