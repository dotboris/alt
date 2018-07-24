use std::env;
use std::fs;
use std::path::*;
use regex::Regex;
use super::CommandVersion;
use config;

lazy_static! {
    static ref COMMAND_VERSION_REGEX: Regex =
        Regex::new(r"^(?P<command>.+[^\d.-])-?(?P<version>\d+(?:\.\d+)*)$").unwrap();
}

fn parse_command_version(bin: PathBuf) -> Option<CommandVersion> {
    let name = String::from(bin.file_name().unwrap().to_str().unwrap());

    COMMAND_VERSION_REGEX.captures(&name)
        .map(|captures| CommandVersion {
            command: String::from(captures.name("command").unwrap().as_str()),
            version: String::from(captures.name("version").unwrap().as_str()),
            path: bin
        })
}

pub fn scan(command: &str) -> Vec<CommandVersion> {
    let path = env::var("PATH").expect("env var PATH is not defined");
    let shim_dir = config::shim_dir();

    env::split_paths(&path)
        .filter(|p| p != &shim_dir)
        .flat_map(|p| fs::read_dir(p).unwrap())
        .map(|bin| bin.unwrap().path())
        .flat_map(|bin| parse_command_version(bin))
        .filter(|c| c.command == command)
        .collect()
}
