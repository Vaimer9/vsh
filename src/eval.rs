use std::env;
use std::env::args;
use std::io::{stdin, stdout, Result, Write};
use std::io;
use std::path::Path;
use std::process::Command;
use std::string::ToString;
use std::process;

pub struct Internalcommand {
    keyword: String,
    args: Vec<String>
}

impl Internalcommand {
    pub fn new(input: String) -> Self {
        let mut splitted = input.trim().split_whitespace();
        let keyword = match splitted.next() {
                Some(x) => x.to_string(),
                None => String::from("") 
        };
        Self {
             keyword,
             args: splitted.map(ToString::to_string).collect::<Vec<String>>()
        }
    }

    pub fn eval(&mut self) -> io::Result<()> {
        match (self.keyword.as_str(), self.args.clone()) {

            ("cd", args) => {
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

            ("", _) => {
                println!();
            },

            ("exit", _) => {
                process::exit(0);
            },

            (x, y) => {
                match Command::new(x).args(y).spawn() {
                    Ok(mut ok) => {
                        ok.wait();
                    },
                    Err(_) => eprintln!("No such command as `{}`", x)
                }
            }
        }
        Ok(())
    }
}
