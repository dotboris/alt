use crate::command_version::CommandVersion;
use crate::command_version::CommandVersionRegistry;
use crate::config;
use crate::shim;
use std::env;
use std::path::*;
use std::process;

pub fn run(command: &str, version: &str, bin: &str) {
    let bin_path = PathBuf::from(bin);

    if !bin_path.exists() {
        println!("File not found: {}", bin);
        process::exit(1);
    }

    let definitions_file_path = config::definitions_file();

    let mut registry = CommandVersionRegistry::load_or_default(&definitions_file_path)
        .expect("TODO: manage command errors better somehow");
    registry.add(CommandVersion::new(command, version, &bin_path));
    registry
        .save(&definitions_file_path)
        .expect("TODO: nice errors maybe");

    shim::make_shim(command, env::current_exe().unwrap().as_path())
        .unwrap_or_else(|err| panic!("failed to create shim for {}: {}", command, err));
}
