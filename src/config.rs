use std::path::{Path, PathBuf};
use std::env;

const DEFAULT_HOME: &'static str = ".config/alt";

pub fn home_dir() -> PathBuf {
    match env::var("ALT_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(DEFAULT_HOME)
        },
    }
}
