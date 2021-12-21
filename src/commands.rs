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

pub fn cd(arg: Option<&String>) {
    match arg {
        Some(dir) => match env::set_current_dir(Path::new(&expand(dir.to_string()))) {
            Ok(_) => (),
            Err(_) => eprintln!("No such directory"),
        },
        None => env::set_current_dir(env::var("HOME").unwrap())
            .expect("Could not go to home directory!"), // HOME will always be set
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
                    // This is only returned if the process was terminated by user
                    None => Err(CommandError::Terminated(127)), // Unfortunately getting the signal is still in nightly
                }
            } else {
                // This is the case in which the command could not be run
                eprintln!("Command could not be executed");
                Err(CommandError::Error)
            }
        }
        Err(_) => {
            eprintln!("vsh: No such command as `{}`", x);
            Err(CommandError::Error)
        }
    }
}

// Expand values. For now this is used only to expand ~ into $HOME,
// but it could easily be modified to be used for variables
fn expand(raw: String) -> String {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\~").unwrap();
    }
    RE.replace_all(&raw, env::var("HOME").unwrap()).to_string()
}
