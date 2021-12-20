use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub const BASE_JSON: &str = r#"
{

}
"#;

lazy_static::lazy_static!{
    pub static ref DATA: String = fetch_data();
}

fn fetch_data() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".vshrc.json");

    let mut data = String::new();
    if path.exists() {
        match File::open(&path) {
            Ok(mut x) => {
                x.read_to_string(&mut data).unwrap();
            }
            Err(_) => {
                eprintln!("vsh: Error Occured while opening `.vshrc.json`");
            }
        }
    } else {
        match File::create(&path) {
            Ok(mut x) => {
                x.write_all(BASE_JSON.as_bytes()).unwrap();
                data = String::from(BASE_JSON);
            }
            Err(_) => eprintln!("Config File could not be created!"),
        }
    }
    data
}
