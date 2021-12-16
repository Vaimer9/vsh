use std::fs::File;
use std::io::Read;
use std::env;

use crate::utils::fetch_data;

use serde_json::Value;
use colored::*;

#[derive(Debug)]
enum Prompt {
    Modern {
        promptchar: String, // prompt charecter, the reason that It is a String is because I want the user to also have prompts such as `->` or `--# `
        color: (usize, usize, usize), // Background color
        text_color: (usize, usize, usize)
        double: bool,       // double line
    },
    Classic {
        promptchar: String, // prompt charecter
        double: bool,       // double line
    },
}

impl Prompt {
    fn new() -> Self {
        // Default Values
        let mut color = (109, 152, 134);
        let mut text_color = (33, 33, 33);
        let mut promptchar = String::from("λ");
        let mut double = false;
        let mut rt = Self::Classic {
            promptchar: promptchar.clone(),
            double,
        };

        if let Ok(y) = Self::json() {
            let x = &y["color"];
            if *x != Value::Null {
                color = (
                    x[0].to_string()
                        .replace("\"", "")
                        .parse::<usize>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[1].to_string()
                        .replace("\"", "")
                        .parse::<usize>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[2].to_string()
                        .replace("\"", "")
                        .parse::<usize>()
                        .expect("Parsing error in `.vsh.json`"),
                );
            }
        }

        if let Ok(y) = Self::json() {
            let x = &y["text_color"];
            if *x != Value::Null {
                color = (
                    x[0].to_string()
                        .replace("\"", "")
                        .parse::<usize>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[1].to_string()
                        .replace("\"", "")
                        .parse::<usize>()
                        .expect("Parsing error in `.vsh.json`"),
                    x[2].to_string()
                        .replace("\"", "")
                        .parse::<usize>()
                        .expect("Parsing error in `.vsh.json`"),
                );
            }
        }

        if let Some(x) = Self::json_value("double") {
            if x.to_uppercase().as_str() =="\"TRUE\"" {
                double = true;
            } else if x.to_uppercase().as_str() == "\"FALSE\"" {
                double = false;
            }
        }

        if let Some(x) = Self::json_value("character") {
            promptchar = x;
        }

        if let Some(x) = Self::json_value("style") {
            match x.to_uppercase().as_str() {
                "\"MODERN\"" => {
                    eprintln!("LESGOO");
                    Self::Modern {
                        promptchar,
                        color,
                        text_color,
                        double,
                    }
                }
                "\"CLASSIC\"" | _ => Self::Classic { promptchar, double },
            }
        } else {
            rt
        }
    }

    fn json_value(name: &str) -> Option<String> {
        let data = fetch_data();
        let v: Value = serde_json::from_str(&data).unwrap();

        if let Value::Null = v[name] {
            None
        } else {
            Some(v[name].to_string())
        }
    }

    fn raw_json() -> std::io::Result<Value> {
        let data = fetch_data();
        let v: Value = serde_json::from_str(&data).unwrap();
        Ok(v)
    }

    fn generate_prompt(&self) -> {

        let lr = String::new();
        match self {
            Self::Modern { ch, color, t_color, double } => {
                if double {
                    format!(
                        "{}{}\n{} ",

                    );
                }
            }
        }
        
    }
}
