use crate::commands;
use std::io;
use std::process;
use std::string::ToString;

pub struct Internalcommand {
    keyword: String,
    args: Vec<String>,
}

impl Internalcommand {
    pub fn new(input: String) -> Self {
        let mut splitted = input.trim().split_whitespace();
        let keyword = match splitted.next() {
            Some(x) => x.to_string(),
            None => String::from(""),
        };
        Self {
            keyword,
            args: splitted.map(ToString::to_string).collect::<Vec<String>>(),
        }
    }

    pub fn eval(&mut self) -> io::Result<()> {
        match (self.keyword.as_str(), self.args.clone()) {
            ("cd", args) => commands::cd(args),

            ("", _) => println!(),

            ("exit", _) => {
                process::exit(0);
            }

            (x, y) => match *x.as_bytes().last().unwrap() as char {
                '/' => commands::cd_for_string(x.to_string()),
                q => commands::neutral(x.to_string(), y),
            },
        }
        Ok(())
    }
}
