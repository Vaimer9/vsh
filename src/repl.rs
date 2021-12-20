use std::env;
use std::fs::File;
use std::io;

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
            eprintln!("vsh: no previous history.");
            if let Err(_) = File::create(format!("{}/.vsh_history", home_dir)) {
                eprintln!("vsh: could not create history file!");
            }
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
                        }
                        Ok(mut com) => match com.call() {
                            Ok(ret_code) => {
                                last_return_val = ret_code;
                            }
                            Err(e) => match e {
                                CommandError::Error(msg) => {
                                    if let Some(st) = msg {
                                        eprintln!("{}", st);
                                        last_return_val = 1;
                                    }
                                }
                                CommandError::Exit(code) => {
                                    if let Err(_) =
                                        rl.save_history(&format!("{}/.vsh_history", home_dir))
                                    {
                                        eprintln!("vsh: could not save command history!")
                                    }
                                    return Ok(code.unwrap_or(0));
                                }
                            },
                        },
                    }
                }
                Err(ReadlineError::Interrupted) => println!(),
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("vsh: an error has occurred: {:?}\n\nPlease report this in https://github.com/xmantle/vsh/issues!", err);
                    break;
                }
            }
            if let Err(_) = rl.save_history(&format!("{}/.vsh_history", home_dir)) {
                eprintln!("vsh: could not save command history!")
            }
        }
        Ok(0)
    }
}
