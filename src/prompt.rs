/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use colored::*;
use serde_derive::Deserialize;

use crate::utils::Config;

pub enum Prompt {
    Modern {
        promptchar: String,
        color: (u8, u8, u8),
        text_color: (u8, u8, u8),
        double: bool,
    },
    Classic {
        promptchar: String,
        text_color: (u8, u8, u8),
        double: bool,
    },
    Arrow,
}

#[derive(Deserialize)]
pub struct PromptConfig {
    style: Option<String>,
    promptchar: Option<String>,
    color: Option<[u8; 3]>,
    text_color: Option<[u8; 3]>,
    double: Option<bool>,
}

impl Prompt {
    pub fn new(data: &Config) -> Self {
        let mut color = (115, 147, 179);
        let mut text_color = (255, 255, 255);
        let mut promptchar = String::from("λ");
        let mut double = false;
        let rt = Self::Classic {
            promptchar: promptchar.clone(),
            text_color,
            double,
        };

        if let Some(prompt) = &data.prompt {
            if let Some(x) = prompt.color {
                color = (x[0], x[1], x[2]);
            }

            if let Some(x) = prompt.text_color {
                text_color = (x[0], x[1], x[2]);
            }

            if let Some(x) = prompt.double {
                double = x;
            }

            if let Some(x) = &prompt.promptchar {
                promptchar = x.clone();
            }

            if let Some(x) = &prompt.style {
                return match x.to_lowercase().as_str() {
                    "modern" => Self::Modern {
                        promptchar,
                        color,
                        text_color,
                        double,
                    },
                    "arrow" => Self::Arrow,
                    "classic" => Self::Classic {
                        // Using "classic" | _ => ... was causing issues
                        promptchar,
                        text_color,
                        double,
                    },
                    _ => Self::Classic {
                        promptchar,
                        text_color,
                        double,
                    },
                };
            } else {
                return rt;
            }
        }
        rt
    }

    pub fn generate_prompt(&self) -> String {
        let current_dir = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        match self {
            Self::Modern {
                promptchar,
                color,
                text_color,
                double,
            } => {
                let backarrow = "".truecolor(color.0, color.1, color.2);
                let forwardarrow = "".truecolor(color.0, color.1, color.2);
                let directory = format!("   {} ", current_dir)
                    .on_truecolor(color.0, color.1, color.2)
                    .truecolor(text_color.0, text_color.1, text_color.2)
                    .bold();
                let pr_char = promptchar
                    .replace("\"", "")
                    .truecolor(color.0, color.1, color.2);

                if *double {
                    format!("{}{}{}\n{} ", backarrow, directory, forwardarrow, pr_char)
                } else {
                    format!("{}{} ", directory, forwardarrow)
                }
            }

            Self::Classic {
                promptchar,
                double,
                text_color,
            } => {
                if *double {
                    format!(
                        "[{}]\n{} ",
                        current_dir.truecolor(text_color.0, text_color.1, text_color.2),
                        promptchar
                    )
                } else {
                    format!(
                        "[{}]{} ",
                        current_dir.truecolor(text_color.0, text_color.1, text_color.2),
                        promptchar
                    )
                }
            }

            Self::Arrow => {
                let pretty_cwd = if current_dir == "/" {
                    "/".bold().truecolor(36, 55, 224)
                } else {
                    // Unwrap free, insured in the first if statement
                    current_dir
                        .split('/')
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap()
                        .bold()
                        .truecolor(36, 55, 224)
                };

                format!("{}  {} ", "➜".bold().truecolor(51, 148, 34), pretty_cwd)
            }
        }
    }
}
