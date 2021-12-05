use std::env;
use std::env::args;
use std::io::{stdin, stdout, Result, Write};
use std::path::Path;
use std::process::Command;
use std::string::ToString;


use ansi_term::{Colour, Style};

pub const shell_prompt: char = 'λ';

pub struct InternalCommand {
    keyword: String,
    args: Vec<String>
}


impl InternalCommand {
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
}

pub fn start_shell() -> std::io::Result<()> {
    loop {
        let current_dir = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();


        println!("| {}", Colour::Red.bold().paint(current_dir));
        print!("|-{} ", shell_prompt);
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let command = InternalCommand::new(input);

        match (command.keyword.as_str(), command.args) {
            ("cd", x) =>{
                match x.iter().next() {
                    Some(e) => {
                        let path = Path::new(e);
                        match env::set_current_dir(path) {
                            Ok(_) => (),
                            Err(_) => println!("No such directory")
                        }
                    }
                    None => eprintln!("Please specify a directory")
                }
                
            }

            ("exit", _) => {
                std::process::exit(0);
            }

            ("", _) => println!(),

            (x, y) => {
                match Command::new(x).args(y).spawn() {
                    Ok(mut ok) => {
                        ok.wait();
                    }
                    Err(error) => println!("Command not found")
                }
            }
            _ => ()
        }
    }
}
