/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::builtins;
use crate::command::Builtin;
use crate::utils::expand;

use std::collections::HashMap;
use std::process::{Command, Stdio, Child};
use std::string::ToString;

pub struct Vshcommand {
    keyword: String,
    args: Vec<String>,
}

pub enum CommandError {
    Error(String),
    Exit,
    Finished(i32),   // If the program finished with a non-zero exit code
    Terminated(i32), // If the program was terminated by the user
}

impl Vshcommand {
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

    pub fn eval(raw: String, aliases: &HashMap<&str, &str>) -> Result<(), CommandError> {
        
        let mut commands = raw.split('|').peekable();
        let mut prev = None;
        let mut status = Ok(());

        while let Some(command) = commands.next() {
            let vshcmd = Self::new(command.to_string());


            status = match (vshcmd.keyword.as_str(), vshcmd.args.clone()) {
                ("cd", args) => builtins::cd::Cd::run(args),

                ("", _) => Ok(()),

                ("exit", _) => Err(CommandError::Exit),

                (x, y) => match *x.as_bytes().last().unwrap() as char {
                    '/' => builtins::cd::Cd::run(vec![x.to_string()]),
                    _ => {

                        let stdin = prev.cloned().map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };

                        let args = y.into_iter().map(expand).collect::<Vec<_>>();
                        if let Some(alias) = &aliases.get(x) {
                            let mut new_x = alias.to_string();

                            for flag in &args {
                                new_x.push_str(&format!(" {}", flag));
                            }

                            return Self::run(new_x, aliases);
                        }
                        return match Command::new(&x)
                            .args(args)
                            .stdout(stdout)
                            .stdin(stdin)
                            .spawn() 
                        {
                            Ok(mut ok) => {
                                prev = Some(ok);

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
                            Err(_) => {
                                prev = None;
                                Err(CommandError::Error(format!("No such command as `{}`", x)))
                            }
                        }
                    }
                },
            };
        }
        status
    }

    pub fn run(x: String, y: &HashMap<&str, &str>) -> Result<(), CommandError> {
        let mut last_return = Ok(());
        for com in x.split(';') {
            last_return = Self::run_linked_commands(com.into(), y);
        }
        last_return
    }

    fn run_command(raw: String, x: &HashMap<&str, &str>) -> Result<(), CommandError> {
        Self::eval(raw, x)
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
