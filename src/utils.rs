use serde_json::Value;

// There will be more don't worry this file is not clutter
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
