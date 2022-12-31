use crate::definitions::Definitions;
use crate::use_file;
use std::env;
use std::fs;
use std::path::*;

pub fn find_selected_version(command: &str) -> Option<String> {
    let file = use_file::find(&env::current_dir().unwrap()).and_then(|path| use_file::load(&path));

    file.as_ref()
        .and_then(|file| file.get(command))
        .map(|version| version.to_owned())
}

pub fn find_system_bin(command: &str) -> Option<PathBuf> {
    let system_path = env::var("PATH").ok()?;
    let current_exe = env::current_exe().and_then(fs::canonicalize).unwrap();

    env::split_paths(&system_path)
        .map(|p| p.join(command))
        .filter(|p| p.exists())
        .map(|p| fs::canonicalize(p).unwrap())
        .find(|p| p != &current_exe)
}

pub fn find_selected_binary(definitions: &Definitions, command: &str) -> Option<PathBuf> {
    match find_selected_version(command) {
        Some(version) => definitions.get_binary(command, &version),
        None => find_system_bin(command),
    }
}
