extern crate toml;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};

const FILE_NAME: &'static str = ".alt.toml";

pub fn find(start: &Path) -> Option<PathBuf> {
    let mut dir = PathBuf::from(start);
    loop {
        let file = dir.join(FILE_NAME);

        if file.is_file() {
            return Some(file);
        } else {
            if dir.parent().is_none() {
                return None;
            } else {
                dir.pop();
            }
        }
    }
}

pub fn find_or_dir(start: &Path) -> PathBuf {
    find(start)
        .unwrap_or_else(|| PathBuf::from(start))
}

pub type UseFile = HashMap<String, String>;

pub fn load(path: &Path) -> UseFile {
    let contents = fs::read(path)
        .expect("failed to read use file");

    toml::from_slice(&contents).unwrap()
}

pub fn save(use_def: &UseFile, path: &Path) -> Result<(), io::Error> {
    let toml = toml::to_string_pretty(use_def)
        .expect("failed to serialize use toml");
    fs::write(path, toml)
}
