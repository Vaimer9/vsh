/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::theme::{
    format::{construct_colored, Context},
    parser::Theme,
};

// This struct is to know what prompt appearance was at STARTUP
pub struct Prompt<'a> {
    theme: Theme<'a>,
}

// This struct is to know CURRENT prompt Info, i.e what the last last command's exit status was
#[derive(Debug)]
pub struct PromptInfo {
    pub terminated: bool,
    pub exit_code: Option<i32>,
}

// This Struct Is to get the info from `.vshrc.toml`

impl<'a> Prompt<'a> {
    pub fn new(theme: Theme<'a>) -> Self {
        Self { theme }
    }

    pub fn generate_prompt(&self, pri: &PromptInfo) -> String {
        // The following lines could not be created into a function due to compiler optimization
        // issue, atleast thats what I think
        let current_dir = {
            let current_path = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
            let home = std::env::var("HOME").unwrap();
            current_path.replace(&home, "~")
        };

        let mut ctx = Context::new();
        ctx.set_var("current_dir", &current_dir);

        construct_colored(&self.theme, ctx)
    }
}

impl PromptInfo {
    pub fn new(terminated: bool, exit_code: Option<i32>) -> Self {
        Self {
            terminated,
            exit_code,
        }
    }

    pub fn default(&mut self) {
        self.terminated = false;
        self.exit_code = None;
    }
}
