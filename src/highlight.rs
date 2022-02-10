extern crate alloc;

use std::borrow::Cow::{self, Borrowed, Owned};

use crate::utils::Config;

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
    pub colores: Colors
}

// Data struct
struct EffectsConfig {
    bold: bool,
    underlined: bool,
    dimmed: bool
}

enum Colors {
    Black(EffectsConfig),
    Red(EffectsConfig),
    Green(EffectsConfig),
    Yellow(EffectsConfig),
    Blue(EffectsConfig),
    Purple(EffectsConfig),
    Cyan(EffectsConfig),
    White(EffectsConfig),
    Truecolor([u8; 3], EffectsConfig)
}

impl PromptEffects {
    pub fn new(config: &Config) -> Self {
        // Default Values for Effects
        let mut (bold, underlined, dimmed) = (false, false, false); 
        let mut truecolors_exist = true;
        let mut truecolors = [200, 0, 0];
        let mut suggestions = String::from("truecolor");


        if let Some(cnf) = config.effects {
            if let Some(x) = cnf.bold {
                bold = x;
            }

            if let Some(x) = cnf.underlined {
                underlined = x;
            }

            if let Some(x) = cnf.dimmed {
                dimmed = x;
            }

            if let Some(x) = cnf.truecolors && Some(y) = cnf.true_suggestion_color {
                if x {
                    truecolors = y;
                }
            }

            if let Some(x) = conf.suggestions_color {
                suggestions = x;
            }
        }

        let effconf = EffectsConfig { bold, underlined, dimmed };
        let mut colors = match suggestions.to_lowercase().as_str() {
            "black"  => Colors::Black(effconf)
            "red"    => Colors::Red(effconf)
            "green"  => Colors::Green(effconf)
            "yellow" => Colors::Yellow(effconf)
            "blue"   => Colors::Blue(effconf)
            "purple" => Colors::Purple(effconf)
            "cyan"   => Colors::Cyan(effconf)
            "white"  => Colors::White(effconf)
            _        => Colors::Truecolor(truecolors, effconf)
        };

    }
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
        Owned("\x1b[2m".to_owned() + hint + "\x1b[m")
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
