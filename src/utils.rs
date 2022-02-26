/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde_derive::Deserialize;

const BASE_CONFIG: &str = r#"
[prompt]
double = false
color = [115, 147, 179]
text_color = [33, 33, 33]
promptchar = "λ"
style = "classic"

[misc]
alias = [
	["", ""]
]

[effects]
underlined = false 
bold = true
dimmed = false
suggestion_color = "red"

truecolors = false 
true_suggestion_color = [255, 0, 0]
"#;

// Config for .vshrc.toml config file
// For example: prompt would mean [prompt]
#[derive(Deserialize, Clone)]
pub struct Config {
    pub prompt: Option<PromptConfig>,
    pub misc: Option<Misc>,
    pub effects: Option<EffectsCtx>,
}

#[derive(Deserialize, Clone)]
pub struct Misc {
    pub alias: Option<Vec<[String; 2]>>,
}

#[derive(Deserialize, Clone)]
pub struct PromptConfig {
    pub style: Option<String>,
    pub promptchar: Option<String>,
    pub color: Option<[u8; 3]>,
    pub text_color: Option<[u8; 3]>,
    pub double: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub struct EffectsCtx {
    pub truecolors: Option<bool>,
    pub underlined: Option<bool>,
    pub bold: Option<bool>,
    pub dimmed: Option<bool>,
    pub suggestion_color: Option<String>,
    pub true_suggestion_color: Option<[u8; 3]>,
}

pub fn fetch_data() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("vsh");
    path.push("config.toml");

    let mut data = String::new();
    if path.exists() {
        match File::open(&path) {
            Ok(mut x) => {
                if x.read_to_string(&mut data).is_err() {
                    eprintln!("vsh: config file is not in UTF-8 encoding and cannot be read");
                }
            }
            Err(_) => {
                eprintln!("vsh: Error Occured while opening config file");
            }
        }
    } else {
        {
            path.pop();
            let dir = &path;
            if let Err(e) = fs::create_dir_all(dir) {
                eprintln!("Could not create configuration directory\n{e}")
            }
            path.push("config.toml");
        } // Have to do this manually as there is no create_file_all() function in std lib
        match File::create(&path) {
            Ok(mut x) => {
                if x.write_all(BASE_CONFIG.as_bytes()).is_err() {
                    eprintln!("vsh: Could not write to config file")
                }
                data = String::from("");
            }
            Err(_) => eprintln!("vsh: Config File could not be created!"),
        }
    }
    data
}

pub fn get_toml(data: String) -> Result<Config, String> {
    match toml::from_str::<Config>(&data) {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_alias(data: &Config) -> HashMap<&str, &str> {
    let mut list: HashMap<&str, &str> = HashMap::new();
    if let Some(misc) = &data.misc {
        if let Some(alias) = &misc.alias {
            for x in alias.iter() {
                list.insert(&x[0], &x[1]);
            }
        }
    }
    list
}

pub fn expand(raw: String) -> String {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\~").unwrap();
    }
    RE.replace_all(&raw, env::var("HOME").unwrap()).to_string()
}
