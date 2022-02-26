/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![warn(unreachable_code)]

use std::env;
use std::fs::File;
use std::io;
use std::process;
use std::thread;

use crate::eval::{CommandError, Internalcommand};
use crate::highlight::PromptEffects;
use crate::prompt::{Prompt, PromptInfo};
use crate::theme::context::Context;
use crate::theme::context::SessionContext;
use crate::theme::context::ThemeContext;
use crate::theme::parser::parse_theme;
use crate::theme::parser::Span;
use crate::utils::{fetch_data, get_alias, get_toml};

use libc::c_int;
use signal_hook::consts::signal::*;
use signal_hook::low_level;

use rustyline::completion::FilenameCompleter;
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{CompletionType, Config, EditMode, Editor};

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

        let theme = config_data.prompt.as_ref().unwrap().theme.as_ref().unwrap();
        let theme = parse_theme(Span::new(&theme)).unwrap().1;

        let mut general_ctx = Context::new();
        general_ctx.from_sub_context(&SessionContext::new());

        loop {
            general_ctx.from_sub_context(&promptinfo);

            let prompt = Prompt::new(theme.clone()).generate_prompt(&general_ctx);

            let helper = PromptEffects {
                completer: FilenameCompleter::new(),
                highlighter: MatchingBracketHighlighter::new(),
                hinter: HistoryHinter {},
                colored_prompt: prompt.clone(),
                validator: MatchingBracketValidator::new(),
                ctx: config_data.clone(),
            };

            rl.set_helper(Some(helper));

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
