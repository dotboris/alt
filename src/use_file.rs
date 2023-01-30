use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FILE_NAME: &str = ".alt.toml";

pub fn find(start: &Path) -> Option<PathBuf> {
    let mut dir = PathBuf::from(start);
    loop {
        let file = dir.join(FILE_NAME);

        if file.is_file() {
            return Some(file);
        } else {
            dir.parent()?;
            dir.pop();
        }
    }
}

pub fn find_or_dir(start: &Path) -> PathBuf {
    find(start).unwrap_or_else(|| start.join(FILE_NAME))
}

pub type UseFile = HashMap<String, String>;

pub fn load(path: &Path) -> Option<UseFile> {
    fs::read_to_string(path)
        .ok()
        .map(|contents| toml::from_str(&contents).unwrap())
}

pub fn save(use_def: &UseFile, path: &Path) -> Result<(), io::Error> {
    let toml = toml::to_string_pretty(use_def).expect("failed to serialize use toml");
    fs::write(path, toml)
}
