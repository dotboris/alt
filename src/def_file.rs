extern crate toml;

use std::collections::HashMap;
use std::fs;
use config;

const DEFS_FILE_NAME: &'static str = "defs.toml";

pub type CommandVersions = HashMap<String, String>;
pub type CommandDefs = HashMap<String, CommandVersions>;

fn command_versions(raw_defs: String, command: &str) -> CommandVersions {
    let all_defs: CommandDefs = toml::from_str(&raw_defs)
        .expect("failed to parse definitions toml");

    all_defs.get(command)
        .map(|r| r.clone())
        .unwrap_or_else(|| CommandVersions::new())
}

pub fn load_for(command: &str) -> CommandVersions {
    let def_file_path = config::home_dir().join(DEFS_FILE_NAME);
    match fs::read_to_string(def_file_path) {
        Ok(contents) => command_versions(contents, command),
        Err(_) => CommandVersions::new(),
    }
}

pub fn load() -> CommandDefs {
    let path = config::home_dir().join(DEFS_FILE_NAME);
    if path.is_file() {
        let bytes = fs::read(path).expect("failed to read defs file");
        toml::from_slice(&bytes).expect("failted to parse defs file toml")
    } else {
        CommandDefs::new()
    }
}
