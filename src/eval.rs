/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::builtins;
use crate::command::Builtin;
use crate::repl::Repl;
use crate::utils::expand;

use std::collections::HashMap;
use std::process::Command;
use std::string::ToString;

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

    pub fn eval(&mut self, aliases: &HashMap<&str, &str>) -> Result<(), CommandError> {
        match (self.keyword.as_str(), self.args.clone()) {
            ("cd", args) => builtins::cd::Cd::run(args),

            ("", _) => Ok(()),

            ("exit", _) => Err(CommandError::Exit),

            (x, y) => match *x.as_bytes().last().unwrap() as char {
                '/' => builtins::cd::Cd::run(vec![x.to_string()]),
                _ => {
                    let args = y.into_iter().map(expand).collect::<Vec<_>>();
                    if let Some(alias) = &aliases.get(x) {
                        let mut new_x = alias.to_string();

                        for flag in &args {
                            new_x.push_str(&format!(" {}", flag));
                        }

                        return Self::run(new_x, aliases);
                    }

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
                                Err(CommandError::Error(
                                    "Command could not be executed".to_string(),
                                ))
                            }
                        }
                        Err(_) => Err(CommandError::Error(format!("No such command as `{}`", x))),
                    }
                }
            },
        }
    }

    pub fn run(x: String, y: &HashMap<&str, &str>) -> Result<(), CommandError> {
        let mut last_return = Ok(());
        for com in x.split(';') {
            last_return = Self::run_linked_commands(com.into(), y);
        }
        last_return
    }

    fn run_command(com: String, x: &HashMap<&str, &str>) -> Result<(), CommandError> {
        Internalcommand::new(com).eval(x)
    }

    fn run_linked_commands(commands: String, x: &HashMap<&str, &str>) -> Result<(), CommandError> {
        for linked_com in commands.split("&&") {
            if let Err(e) = Self::run_command(linked_com.to_string(), x) {
                return Err(e);
            }
        }
        Ok(())
    }
}
