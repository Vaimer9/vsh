/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::utils::{fetch_data, BASE_JSON};

use colored::*;
use serde_json::Value;

#[derive(Debug)]
pub enum Prompt {
    Modern {
        promptchar: String, // prompt charecter, the reason that It is a String is because I want the user to also have prompts such as `->` or `--# `
        color: (u8, u8, u8), // Background color
        text_color: (u8, u8, u8),
        double: bool, // double line
    },
    Classic {
        promptchar: String, // prompt charecter
        double: bool,       // double line
    },
}

impl Prompt {
    pub fn new() -> Self {
        // Default Values
        let mut color = (109, 152, 134);
        let mut text_color = (33, 33, 33);
        let mut promptchar = String::from("λ");
        let mut double = false;
        let rt = Self::Classic {
            promptchar: promptchar.clone(),
            double,
        };

        if let Ok(y) = Prompt::raw_json() {
            let x = &y["color"];
            if *x != Value::Null {
                color = (
                    x[0].to_string()
                        .replace("\"", "")
                        .parse::<u8>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[1].to_string()
                        .replace("\"", "")
                        .parse::<u8>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[2].to_string()
                        .replace("\"", "")
                        .parse::<u8>()
                        .expect("Parsing error in `.vsh.json`"),
                );
            }
        }

        if let Ok(y) = Prompt::raw_json() {
            let x = &y["text_color"];
            if *x != Value::Null {
                text_color = (
                    x[0].to_string()
                        .replace("\"", "")
                        .parse::<u8>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[1].to_string()
                        .replace("\"", "")
                        .parse::<u8>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[2].to_string()
                        .replace("\"", "")
                        .parse::<u8>()
                        .expect("Parsing error in `.vsh.json`"),
                );
            }
        }

        if let Some(x) = Prompt::json_value("double") {
            if x.to_uppercase().as_str() == "\"TRUE\"" {
                double = true;
            } else if x.to_uppercase().as_str() == "\"FALSE\"" {
                double = false;
            }
        }

        if let Some(x) = Prompt::json_value("character") {
            promptchar = x;
        }

        if let Some(x) = Prompt::json_value("style") {
            match x.to_uppercase().as_str() {
                "\"MODERN\"" => Self::Modern {
                    promptchar,
                    color,
                    text_color,
                    double,
                },
                "\"CLASSIC\"" => Self::Classic { promptchar, double },
                x => {
                    eprintln!(
                        "vsh: Error Parsing `.vshrc.json`\nNo such theme as \"{}\"",
                        x
                    );
                    Self::Classic { promptchar, double }
                }
            }
        } else {
            rt
        }
    }

    fn json_value(name: &str) -> Option<String> {
        let data = fetch_data();

        match serde_json::from_str::<Value>(&data) {
            Ok(v) => {
                if Value::Null == v[name] {
                    None
                } else {
                    Some(v[name].to_string())
                }
            }
            Err(e) => {
                eprintln!("vsh: Error parsing data\n{}", e);
                Some(String::from("{}"))
            }
        }
    }

    fn raw_json() -> std::io::Result<Value> {
        let data = fetch_data();
        match serde_json::from_str(&data) {
            Ok(a) => Ok(a),
            Err(x) => {
                eprintln!("vsh: Error parsing data\n{}", x);
                Ok(serde_json::Value::String(BASE_JSON.to_string()))
            }
        }
    }

    pub fn generate_prompt(&self) -> String {
        let current_dir = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(); // Won't panic
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
            Self::Classic { promptchar, double } => {
                if *double {
                    format!("[{}]\n{} ", current_dir, promptchar)
                } else {
                    format!("[{}]{} ", current_dir, promptchar)
                }
            }
        }
    }
}
