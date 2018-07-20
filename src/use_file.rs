extern crate toml;

use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf, Path};

const FILE_NAME: &'static str = ".alt.toml";

pub fn find(mut dir: PathBuf) -> Option<PathBuf> {
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

pub type UseFile = HashMap<String, String>;

pub fn load(path: &Path) -> UseFile {
    let contents = fs::read(path)
        .expect("failed to read use file");

    toml::from_slice(&contents).unwrap()
}
