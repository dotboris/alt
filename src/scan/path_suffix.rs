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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_only_should_not_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python"));
        assert_eq!(None, res)
    }

    #[test]
    fn command_with_simple_number_suffix_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python2"));
        assert_eq!(res, Some(CommandVersion {
            command: String::from("python"),
            version: String::from("2"),
            path: PathBuf::from("/usr/bin/python2"),
        }))
    }

    #[test]
    fn command_with_version_suffix_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python2.7"));
        assert_eq!(res, Some(CommandVersion {
            command: String::from("python"),
            version: String::from("2.7"),
            path: PathBuf::from("/usr/bin/python2.7"),
        }))
    }

    #[test]
    fn command_with_version_suffix_and_dash_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/ruby-2.5"));
        assert_eq!(res, Some(CommandVersion {
            command: String::from("ruby"),
            version: String::from("2.5"),
            path: PathBuf::from("/usr/bin/ruby-2.5"),
        }))
    }

    #[test]
    fn command_with_text_suffix_should_not_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python-config"));
        assert_eq!(res, None);
    }

    #[test]
    fn command_trailing_period_in_suffix_should_not_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/something-2.1."));
        assert_eq!(res, None);
    }
}
