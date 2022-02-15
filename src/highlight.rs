/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use std::borrow::Cow::{self, Borrowed, Owned};

use crate::utils::Config;

use colored::*;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::Context;
use rustyline_derive::Helper;

// Holds all data for Highlighting, bracket validation and completion
// Used by rustyline
#[derive(Helper)]
pub struct PromptEffects {
    pub completer: FilenameCompleter,
    pub highlighter: MatchingBracketHighlighter,
    pub validator: MatchingBracketValidator,
    pub hinter: HistoryHinter,
    pub colored_prompt: String,
    pub ctx: Config,
}

impl Completer for PromptEffects {
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

impl Hinter for PromptEffects {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for PromptEffects {
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
        let mut estr = hint.red();

        if let Some(eff) = &self.ctx.effects {
            if eff.underlined == Some(true) {
                estr = estr.underline();
            } else if eff.bold == Some(true) {
                estr = estr.bold();
            } else if eff.dimmed == Some(true) {
                estr = estr.dimmed();
            }

            if let Some(x) = &eff.suggestion_color {
                match x.to_lowercase().as_str() {
                    "black" => estr = estr.black(),
                    "green" => estr = estr.green(),
                    "yellow" => estr = estr.yellow(),
                    "blue" => estr = estr.blue(),
                    "purple" => estr = estr.purple(),
                    "cyan" => estr = estr.cyan(),
                    "white" => estr = estr.white(),
                    "red" | _ => estr = estr.red(),
                }
            }

            if let Some(x) = eff.truecolors {
                if x {
                    if let Some(y) = eff.true_suggestion_color {
                        // estr = estr.clear(); // As both truecolors and suggestions color might exist we need to clear
                        estr = estr.truecolor(y[0], y[1], y[2]);
                    }
                }
            }
        }

        Owned(format!("{}", estr))
        // Owned("\x1b[2m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for PromptEffects {
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
