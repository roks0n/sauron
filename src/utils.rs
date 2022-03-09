use url::Url;

pub fn get_homedir() -> String {
    match home::home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("Impossible to get your home directory"),
    }
}

pub fn is_valid_url(val: &str) -> bool {
    Url::parse(val).is_ok()
}
