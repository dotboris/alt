use super::CommandVersion;
use glob::glob;
use std::path::*;

const HOMEBREW_GLOB: &'static str = "/usr/local/opt/*@*/bin/*";

fn extract_command_and_version(path: &Path) -> (String, String) {
    let parts: Vec<_> = path.components().collect();

    let command = match parts[6] {
        Component::Normal(part) => String::from(part.to_str().unwrap()),
        _ => panic!(),
    };
    let version = match parts[4] {
        Component::Normal(part) => {
            let package = part.to_str().unwrap();
            let version = package.split('@').last().unwrap();
            String::from(version)
        },
        _ => panic!(),
    };

    (command, version)
}

fn parse_version_path(path: PathBuf) -> CommandVersion {
    let (command, version) = extract_command_and_version(&path);

    CommandVersion {
        command: command,
        version: version,
        path: path,
    }
}

pub fn scan(command: &str) -> Vec<CommandVersion> {
    glob(HOMEBREW_GLOB)
        .unwrap()
        .flat_map(|x| x)
        .map(parse_version_path)
        .filter(|c| c.command == command)
        .collect()
}
