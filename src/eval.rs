/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::commands;
use crate::command;
use crate::command::CommandStructure;

use std::string::ToString;
use std::process::Command;

pub struct Internalcommand {
    keyword: String,
    args: Vec<String>,
}

pub enum CommandError {
    Error(String),
    Exit,
    Finished(i32),   // If the program finished with a non-zero exit code
    Terminated(i32), // If the program was terminated by the user
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

    pub fn eval(&mut self) -> Result<(), CommandError> {
        match (self.keyword.as_str(), self.args.clone()) {
            ("cd", args) => commands::cd::Cd::run(args),
            ("", _) => {
                println!();
                Ok(())
            },
            ("exit", _) => {
                return Err(CommandError::Exit); 
            }
            (x, y) => {
                match *x.as_bytes().last().unwrap() as char {
                    '/' => commands::cd::Cd::run(vec![x.to_string()]),
                    _ => {
                        let args = y.into_iter().map(command::expand).collect::<Vec<_>>();
                        match Command::new(&x).args(args).spawn() {
                            Ok(mut ok) => {
                                if let Ok(status) = ok.wait() {
                                    match status.code() {
                                        Some(code) => {
                                            if code > 0 {
                                                Err(CommandError::Finished(code))
                                            } else {
                                                Ok(())
                                            }
                                        }
                                        None => Err(CommandError::Terminated(127)), 
                                    }
                                } else {
                                    Err(CommandError::Error("Command could not be executed".to_string()))
                                }
                            }
                            Err(_) => {
                                Err(CommandError::Error(format!("No such command as `{}`", x)))
                            }
                        }
                    }
                }
            }
        }
    }
}
