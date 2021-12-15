use serde_json::Value;

fn get_json() -> Value {
    let v: Value = serde_json::from_str(&fetch_data);
}

fn validate_json_value(x: Option<&str>) -> Option<String> {
    match x {
         => None,
        Some(x) => return x.to_string()
    }
}

fn fetch_data() -> String {
    path = format!("{}/.vshrc.json", env::var("HOME").unwrap());
    let mut data = String::new();
    if Path::new(&path).exists() {
        File::open(&path).read_to_string(&mut data);
    } else {
        match File::create(&path) {
            Ok(_) => data = fetch_data(),
            Err(_) => eprintln!("Config File could not be created!")
        }
    }
    data
}
