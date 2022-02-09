/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![warn(unreachable_code)]

extern crate alloc;

use std::borrow::Cow::{self, Borrowed, Owned};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process;

// use std::{
//     borrow::Cow::{self, Borrowed, Owned},
//     collections::HashMap,
//     env,
//     fs::File
// }

use crate::eval::{CommandError, Internalcommand};
use crate::prompt::{Prompt, PromptInfo};
use crate::utils::{fetch_data, get_alias, get_toml};

use libc::c_int;
use signal_hook::consts::signal::*;
use signal_hook::low_level;
use std::thread;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{CompletionType, Config, Context, EditMode, Editor};
use rustyline_derive::Helper;

#[cfg(feature = "extended-siginfo")]
type Signals =
    signal_hook::iterator::SignalsInfo<signal_hook::iterator::exfiltrator::origin::WithOrigin>;

#[cfg(not(feature = "extended-siginfo"))]
use signal_hook::iterator::Signals;

#[derive(Helper)]
struct PromptHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for PromptHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for PromptHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for PromptHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[2m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for PromptHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

pub struct Repl;

impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_shell(&mut self) -> io::Result<()> {
        let prconf = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .output_stream(OutputStreamType::Stdout)
            .build();

        let mut rl = Editor::with_config(prconf);

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

            let helper = PromptHelper {
                completer: FilenameCompleter::new(),
                highlighter: MatchingBracketHighlighter::new(),
                hinter: HistoryHinter {},
                colored_prompt: prompt.clone(),
                validator: MatchingBracketValidator::new(),
            };
            rl.set_helper(Some(helper));

            print!("\r\n");
            if let Err(flusherr) = std::io::stdout().flush() {
                eprintln!("vsh: Could not flush stdout\n{flusherr}");
            }

            let readline = rl.readline(prompt.as_str());

            match readline {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());

                    if let Err(e) = Internalcommand::run(x, &aliases) {
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

    
}
