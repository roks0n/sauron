use std::fs;
use std::fs::{File};
use std::path::Path;
use crate::utils::get_homedir;

const STORAGE_PATH: &str = ".local/share/sauron-watcher/";
const STORAGE_FILE: &str = "data.json";

pub fn get_storage() -> File {
    let home_dir = get_homedir();
    
    let storage_path = &format!("{}/{}", home_dir, STORAGE_PATH);
    fs::create_dir_all(storage_path).unwrap();

    let file;
    let storage_file_path = &format!("{}/{}/{}", home_dir, STORAGE_PATH, STORAGE_FILE);
    if !Path::new(storage_file_path).exists() {
        file = File::create(storage_file_path).unwrap();
    } else {
        file = File::options().read(true).write(true).open(storage_file_path).unwrap();
    }

    file
}