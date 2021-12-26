/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub const BASE_JSON: &str = r#"
{

}
"#;

pub fn fetch_data() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".vshrc.json");

    let mut data = String::new();
    if path.exists() {
        match File::open(&path) {
            Ok(mut x) => {
                if let Err(_) = x.read_to_string(&mut data) {
                    eprintln!("vsh: config file is not in UTF-8 encoding and cannot be read");
                }
            }
            Err(_) => {
                eprintln!("vsh: Error Occured while opening config file");
            }
        }
    } else {
        match File::create(&path) {
            Ok(mut x) => {
                x.write_all(BASE_JSON.as_bytes()).unwrap();
                if let Err(_) = x.write_all(BASE_JSON.as_bytes()) {
                    eprintln!("vsh: Could not write to config file")
                }
                data = String::from(BASE_JSON);
            }
            Err(_) => eprintln!("vsh: Config File could not be created!"),
        }
    }
    data
}
