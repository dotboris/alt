use std::path::*;
use std::env;
use use_file;
use config;

pub fn find_selected_version(command: &str) -> Option<String> {
    let file = use_file::find(env::current_dir().unwrap())
        .map(|path| use_file::load(&path));

    file.as_ref()
        .and_then(|file| file.get(command))
        .map(|version| version.to_owned())
}

pub fn find_system_bin(command: &str) -> Option<PathBuf> {
    let shim_dir = config::shim_dir();
    let path = env::var("PATH").expect("env var PATH is not defined");
    let paths: Vec<_> = env::split_paths(&path)
        .filter(|p| p != &shim_dir)
        .collect();

    paths.iter()
        .find(|p| p.join(command).exists())
        .map(|p| p.join(command))
}
