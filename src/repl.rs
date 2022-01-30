/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![warn(unreachable_code)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::process;

use crate::eval::{CommandError, Internalcommand};
use crate::prompt::{Prompt, PromptInfo};
use crate::utils::{fetch_data, get_alias, get_toml};

use libc::c_int;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use signal_hook::consts::signal::*;
use signal_hook::low_level;
use std::thread;

#[cfg(feature = "extended-siginfo")]
type Signals =
    signal_hook::iterator::SignalsInfo<signal_hook::iterator::exfiltrator::origin::WithOrigin>;

#[cfg(not(feature = "extended-siginfo"))]
use signal_hook::iterator::Signals;

pub struct Repl;

impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_shell(&mut self) -> io::Result<()> {
        let mut rl = Editor::<()>::new();
        let home_dir = env::var("HOME").unwrap();

        const SIGNALS: &[c_int] = &[SIGTSTP, SIGINT];
        let mut signals = Signals::new(SIGNALS).expect("Failed to create signals struct");

        let _handler = signals.handle();
        let mut promptinfo = PromptInfo::new(false, None);

        if rl
            .load_history(&format!("{}/.vsh_history", home_dir))
            .is_err()
        {
            eprintln!("vsh: No previous history.");
            if File::create(format!("{}/.vsh_history", home_dir)).is_err() {
                eprintln!("vsh: Could not create history file!");
            }
        }

        thread::spawn(move || {
            for signal in signals.forever() {
                match signal {
                    SIGTSTP => (), // ctrlz
                    SIGINT => (),  // ctrlc
                    _ => low_level::emulate_default_handler(signal).unwrap(),
                }
            }
        });

        let config_data = match get_toml(fetch_data()) {
            Ok(x) => x,
            Err(err) => {
                println!("{:?}", err);
                get_toml(String::from("")).unwrap() // Unwrap free
            }
        };

        let aliases = get_alias(&config_data);

        loop {
            let prompt = Prompt::new(&config_data).generate_prompt(&promptinfo);
            let readline = rl.readline(prompt.as_str());

            match readline {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());

                    if let Err(e) = Self::run(x, &aliases) {
                        match e {
                            CommandError::Exit => {
                                if rl
                                    .save_history(&format!("{}/.vsh_history", home_dir))
                                    .is_err()
                                {
                                    eprintln!("vsh: Could not save command history");
                                }
                                process::exit(0);
                            }
                            CommandError::Error(x) => {
                                eprintln!("vsh: {}", x);
                                promptinfo.default();
                            }
                            CommandError::Terminated(_) => {
                                println!("\r");
                                promptinfo.terminated = true;
                                promptinfo.exit_code = None;
                            }
                            CommandError::Finished(code) => {
                                promptinfo.terminated = false;
                                promptinfo.exit_code = Some(code);
                            }
                        }
                    } else {
                        promptinfo.default();
                    }
                }
                Err(ReadlineError::Interrupted) => println!(),
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("vsh: Unexpected Error, please report the error on: https://github.com/xmantle/vsh/issues \n{:?}", err);
                    break;
                }
            }
            if rl
                .save_history(&format!("{}/.vsh_history", home_dir))
                .is_err()
            {
                eprintln!("vsh: Could not save command history");
            }
        }
        Ok(())
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
