use std::env;
use std::io::{stdin, stdout, Result, Write};
use std::path::Path;
use std::process::Command;

use ansi_term::{Colour, Style};

pub const shell_prompt: char = 'λ';

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

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("No Such File or directory");
                }
            }
            "exit" => {
                std::process::exit(0);
            }
            command => match Command::new(command).args(args).spawn() {
                Ok(mut x) => {
                    x.wait();
                    ()
                }
                Err(_) => {
                    println!("No such command or file");
                }
            },
        }
    }
}
