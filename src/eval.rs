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

    pub fn eval(raw: String, aliases: &HashMap<&str, &str> ) -> Result<(), CommandError> {
        let vshcmd = Self::new(raw);
            
        match (vshcmd.keyword.as_str(), vshcmd.args.clone()) {
            ("cd", y) => builtins::cd::Cd::run(y),

            ("", _) => Ok(()),

            ("exit", _) => Err(CommandError::Exit),

            (x, y) => {
                if '|' == *x.as_bytes().last().unwrap() as char {
                    return builtins::cd::Cd::run(y);
                }

                let args = y.into_iter().map(expand).collect::<Vec<_>>();
                
                /// Look for alias in keyword
                /// if found then run command again with keyword replaced with the alias
                if let Some(alias) = &aliases.get(x) {
                    let mut new_x = alias.to_string();

                    /// Add the arguments passed in as well
                    for flags in &args {
                        new_x.push_str(&format!(" {}", flags));
                    }

                    Self::run(new_x, aliases);
                }
                
                let stdin = Stdio::inherit();
                let stdout = Stdio::inherit();

                /// Execute the command and store its info as a Child
                let mut child = Self::exec(x.to_string(), args, stdin, stdout)?;

                // previous = Some(child);
                return Self::get_status(&mut child);
            }
        }
    }

    pub fn run(x: String, y: &HashMap<&str, &str>) -> Result<(), CommandError> {
        let mut last_return = Ok(());
        for com in x.split(';') {
            last_return = Self::run_linked_commands(com.into(), y);
        }
        last_return
    }

    fn call_eval(raw: String, x: &HashMap<&str, &str>) -> Result<(), CommandError> {
        Self::eval(raw, x)
    }

    fn run_linked_commands(commands: String, x: &HashMap<&str, &str>) -> Result<(), CommandError> {
        for linked_com in commands.split("&&") {
            if let Err(e) = Self::call_eval(linked_com.to_string(), x) {
                return Err(e);
            }
        }
        Ok(())
    }

    fn exec(keyword: String, args: Vec<String>, stdout: Stdio, stdin: Stdio) -> Result<Child, CommandError> {
        match Command::new(&keyword)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
        {
            Ok(ok) => Ok(ok),
            Err(_) => Err(CommandError::Error(format!("No such command as `{keyword}`")))
        }
    }

    fn get_status(child: &mut Child) -> Result<(), CommandError> {
        match child.wait() {
            Ok(status) => {
                match status.code() {
                    Some(code) => {
                        if code > 0 {
                            Err(CommandError::Finished(code))
                        } else {
                            Ok(())
                        }
                    }
                    None => Err(CommandError::Terminated(127))
                }
            }
            Err(_) => Err(CommandError::Error("Command could not be executed".to_string()))
        }
    }
}
