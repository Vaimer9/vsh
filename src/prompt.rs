use crate::utils::{fetch_data, BASE_JSON};

use colored::*;
use serde_json::Value;

#[derive(Debug)]
pub enum Prompt {
    Modern {
        prompt: String,
        color: (u8, u8, u8), // Background color
        text_color: (u8, u8, u8),
        err_color: (u8, u8, u8), // Colour when a process returns a non-zero exit code
        double: bool, // double line
    },
    Classic {
        promptchar: String, // prompt charecter
        double: bool,       // double line
    },
}

fn get_triple_val(data: &Value, wanted: &str) -> Result<(u8, u8, u8), String> {
    let x = &data[wanted];
    if *x == Value::Null {
        return Err(format!("Value {} is null in `.vshrc.json`!", wanted));
    }
        let val0 = x[0].to_string()
                .replace("\"", "")
                .parse::<u8>().map_err(|_| format!("Value {} is not parseable!", x[0].to_string()))?;
        let val1 = x[1].to_string()
                .replace("\"", "")
                .parse::<u8>().map_err(|_| format!("Value {} is not parseable!", x[1].to_string()))?;
        let val2 = x[2].to_string()
                .replace("\"", "")
                .parse::<u8>().map_err(|_| format!("Value {} is not parseable!", x[2].to_string()))?;
        Ok((val0, val1, val2))
}

impl Prompt {
    pub fn new() -> Self {
        // Default Values
        let mut color = (109, 152, 134);
        let mut text_color = (33, 33, 33);
        let mut prompt_str = String::from("λ");
        let mut err_color = (217, 33, 33);
        let mut double = false;
        let rt = Self::Classic {
            promptchar: prompt_str.clone(),
            double,
        };

        if let Ok(y) = Prompt::raw_json() {
            if let Ok(val) = get_triple_val(&y, "color") {
                color = val;
            }
            if let Ok(val) = get_triple_val(&y, "text_color") {
                text_color = val;
            }
            if let Ok(val) = get_triple_val(&y, "err_color") {
                err_color = val;
            }
        }

        if let Some(x) = Prompt::json_value("double") {
            double = x.parse::<bool>().unwrap_or(false)
        }

        if let Some(x) = Prompt::json_value("character") {
            prompt_str = x;
        }

        // TODO: Changeable PS1/theme
        if let Some(x) = Prompt::json_value("style") {
            match x.to_uppercase().as_str() {
                "\"MODERN\"" => Self::Modern {
                    prompt: prompt_str,
                    color,
                    text_color,
                    err_color,
                    double,
                },
                "\"CLASSIC\"" => Self::Classic { promptchar: prompt_str, double },
                x => {
                    eprintln!(
                        "vsh: Error Parsing `.vshrc.json`\nNo such theme as \"{}\"",
                        x
                    );
                    Self::Classic { promptchar: prompt_str, double }
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

    pub fn generate_prompt(&self, last_proc_num: i32) -> String {
        let current_dir = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(); // Won't panic
        match self {
            Self::Modern {
                prompt: promptchar,
                color,
                text_color,
                err_color,
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

                let err_msg = format!("[{}]", last_proc_num)
                    .truecolor(err_color.0, err_color.1, err_color.2).bold();

                if *double {
                    if last_proc_num != 0 {
                        format!("{}{}{} {}\n{} ", backarrow, directory, forwardarrow, err_msg, pr_char)
                    } else {
                        format!("{}{}{}\n{} ", backarrow, directory, forwardarrow, pr_char)
                    }
                } else {
                    if last_proc_num != 0 {
                        format!("{}{} {} ", directory, forwardarrow, err_msg)
                    } else {
                        format!("{}{} ", directory, forwardarrow)
                    }
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
