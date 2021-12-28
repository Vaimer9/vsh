/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use lazy_static::lazy_static;
use std::env;
use std::path::Path;
use std::process::Command;

use crate::eval::CommandError;

pub trait CommandStructure {
    fn name() ->  &'static str;

    fn about() -> &'static str;

    fn flags() -> BtreeMap<&' static str, &' static str>

    fn examples() -> [&'static str; 3];

    fn run(Vec<String>) -> Result<(), CommandError>;

    fn help(&self) -> &'static str {
        format!(
            "{}\nAbout: \n{}\nExamples: \n1. {}\n2. {}\n3. {}",
            self.name(),
            self.about(),
            self.examples()[0],
            self.examples()[1],
            self.examples()[2]
        ).as_str()
    }
}

pub fn neutral(x: String, y: Vec<String>) -> Result<(), CommandError> {
    let args = y.into_iter().map(expand).collect::<Vec<_>>();
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

fn expand(raw: String) -> String {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\~").unwrap();
    }
    RE.replace_all(&raw, env::var("HOME").unwrap()).to_string()
}
