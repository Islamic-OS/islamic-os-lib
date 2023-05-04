use home::home_dir;
use std::path::Path;
// use toml::Value;

// --- USING LEVELDB ---

use rusty_leveldb::{Options, DB};

pub fn is_config_available() -> bool {
    let home_dir = match home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("Could not get home directory!"),
    };
    // let dir = format!("{}/.config/ism.config.toml", home_dir);
    let dir = format!("{}/.config/ism.db", home_dir);
    let config_path = Path::new(&dir);

    config_path.exists()
}

pub fn create_new_config() {
    let home_dir = match home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("Could not get home directory!"),
    };
    // let dir = format!("{}/.config/ism.config.toml", home_dir);
    let dir = format!("{}/.config/ism.db", home_dir);
    let config_path = Path::new(&dir);

    //     let mut config_file = File::create(config_path).unwrap();
    //     let config_string = r#"[location]
    // latitude = 0.0
    // longitude = 0.0

    // [salah_cfg]
    // method = "UmmAlQura"
    // madhab = "Hanafi"
    // "#;
    //     config_file.write_all(config_string.as_bytes()).unwrap();

    let mut options = Options::default();
    options.create_if_missing = true;
    let mut db = DB::open(config_path, options).unwrap();

    db.put(b"location_latitude", b"0.0").unwrap();
    db.put(b"location_longitude", b"0.0").unwrap();
    db.put(b"salahcfg_method", b"UmmAlQura").unwrap();
    db.put(b"salahcfg_madhab", b"Hanafi").unwrap();

    db.close().unwrap();
}

pub fn get_config() -> DB {
    // Get config file from ~/.config/ism.config.toml
    let home_dir = match home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("Could not get home directory!"),
    };
    // let dir = format!("{}/.config/ism.config.toml", home_dir);
    let dir = format!("{}/.config/ism.db", home_dir);
    let config_path = Path::new(&dir);

    // let mut config_file = File::open(config_path).unwrap();
    // let mut config_string = String::new();
    // config_file.read_to_string(&mut config_string).unwrap();
    // let config = config_string.parse::<Value>().unwrap();

    // config

    #[allow(unused_mut)]
    let mut options = Options::default();
    #[allow(unused_mut)]
    let mut db = DB::open(config_path, options).unwrap();

    db
}

pub fn display_current_location() {
    let mut config = get_config();

    let latitude = config.get(b"location_latitude").unwrap();
    let longitude = config.get(b"location_longitude").unwrap();

    println!(
        "Current location: {}, {}",
        String::from_utf8(latitude).unwrap(),
        String::from_utf8(longitude).unwrap()
    );
}

pub fn update_config_value(key: &str, value: &str) {
    let mut config = get_config();

    config.put(key.as_bytes(), value.as_bytes()).unwrap();

    config.close().unwrap();
}
