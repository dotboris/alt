use std::path::*;
use std::env;
use use_file;
use std::fs;

pub fn find_selected_version(command: &str) -> Option<String> {
    let file = use_file::find(&env::current_dir().unwrap())
        .and_then(|path| use_file::load(&path));

    file.as_ref()
        .and_then(|file| file.get(command))
        .map(|version| version.to_owned())
}

fn realpath_equal(a: &Path, b: &Path) -> bool {
    fs::canonicalize(&a).unwrap() == fs::canonicalize(&b).unwrap()
}

pub fn find_system_bin(command: &str, system_path: &str, current_exe: &Path) -> Option<PathBuf> {
    env::split_paths(&system_path)
        .map(|p| p.join(command))
        .filter(|p| p.exists())
        .filter(|p| !realpath_equal(&p, &current_exe))
        .next()
}
