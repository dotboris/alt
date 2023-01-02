use crate::config;
use std::collections::HashMap;
use std::fs;
use std::path::*;

const DEFS_FILE_NAME: &str = "defs.toml";

pub type CommandVersions = HashMap<String, PathBuf>;
pub type CommandDefs = HashMap<String, CommandVersions>;

pub fn load() -> CommandDefs {
    let path = config::home_dir().join(DEFS_FILE_NAME);
    if path.is_file() {
        let bytes = fs::read(path).expect("failed to read defs file");
        toml::from_slice(&bytes).expect("failed to parse defs file toml")
    } else {
        CommandDefs::new()
    }
}
