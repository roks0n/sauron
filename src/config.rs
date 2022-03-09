use std::fs;
use std::fs::{File};
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::utils::get_homedir;

const CONFIG_PATH: &str = ".config/sauron-watcher/";
const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub statsd_host: String,
    pub statsd_prefix: String,
}

pub fn get_config_file() -> File {
    let home_dir = get_homedir();
    
    let config_path = &format!("{}/{}", home_dir, CONFIG_PATH);
    fs::create_dir_all(config_path).unwrap();

    let file: File;
    let config_file_path = &format!("{}/{}/{}", home_dir, CONFIG_PATH, CONFIG_FILE);

    if !Path::new(config_file_path).exists() {
        file = File::create(config_file_path).unwrap();
    } else {
        file = File::options().read(true).write(true).open(config_file_path).unwrap();
    }

    file
}

pub fn get_config() -> Config {
    let mut file = get_config_file();
    let mut config_string= String::new();
    file.read_to_string(&mut config_string).unwrap();

    let config_data: crate::config::Config = toml::from_str(&mut config_string).unwrap();
    config_data
}