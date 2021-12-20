use std::env;
use std::fs::File;
use std::io;
use std::process;

use crate::eval::InternalCommand;
use crate::{eval::CommandError, prompt::Prompt};

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {}

impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_shell(&mut self) -> io::Result<i32> {
        let mut rl = Editor::<()>::new();
        let home_dir = env::var("HOME").unwrap(); // There should be a HOME dir so no need to worry about this unwrap

        if rl
            .load_history(&format!("{}/.vsh_history", home_dir))
            .is_err()
        {
            eprintln!("No previous history.");
            File::create(format!("{}/.vsh_history", home_dir)).expect("Can't create history File!");
        }

        let prompt = Prompt::new();

        // TODO: Integrate with prompt
        let mut last_return_val = 0;

        loop {
            let readline = rl.readline(&prompt.generate_prompt(last_return_val));

            match readline {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());
                    match InternalCommand::new(x) {
                        Err(e) => {
                            match e {
                                CommandError::Error(s) => {
                                    if let Some(msg) = s {
                                        eprintln!("{}", msg);
                                    }
                                    last_return_val = 127;
                                }
                                _ => continue, // TODO: What should happen if an error is returned?
                            }
                        },
                        Ok(mut com) => {
                            match com.call() {
                                Ok(ret_code) => {
                                    last_return_val = ret_code;
                                },
                                Err(e) => {
                                    match e {
                                        CommandError::Error(msg) => {
                                            if let Some(st) = msg {
                                                eprintln!("{}", st);
                                                last_return_val = 1;
                                            }
                                        },
                                        CommandError::Exit(code) => {
                                            rl.save_history(&format!("{}/.vsh_history", home_dir))
                                                .expect("Couldn't Save History");
                                            return Ok(code.unwrap_or(0))
                                        },
                                        CommandError::Terminated(code) => {
                                            last_return_val = code;
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => println!(),
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
            rl.save_history(&format!("{}/.vsh_history", home_dir))
                .expect("Couldn't Save History");
        }
        Ok(0)
    }
}
