use std::path::*;
use std::env;
use use_file;

pub fn find_selected_version(command: &str) -> Option<String> {
    let file = use_file::find(env::current_dir().unwrap())
        .map(|path| use_file::load(&path));

    file.as_ref()
        .and_then(|file| file.get(command))
        .map(|version| version.to_owned())
}

pub fn find_system_bin(command_name: &str) -> Option<PathBuf> {
    None
}
