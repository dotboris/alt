use crate::command_version::CommandVersion;
use crate::config;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::path::*;

lazy_static! {
    static ref COMMAND_VERSION_REGEX: Regex =
        Regex::new(r"^(?P<command>[^\d.-]+)-?(?P<version>\d+(?:\.\d+)*)$").unwrap();
}

fn parse_command_version(bin: PathBuf) -> Option<CommandVersion> {
    let name = String::from(bin.file_name().unwrap().to_str().unwrap());

    COMMAND_VERSION_REGEX.captures(&name).map(|captures| {
        CommandVersion::new(
            captures.name("command").unwrap().as_str(),
            captures.name("version").unwrap().as_str(),
            &bin,
        )
    })
}

pub fn scan(command: &str) -> Vec<CommandVersion> {
    let path = env::var("PATH").expect("env var PATH is not defined");
    let shim_dir = config::shim_dir();

    env::split_paths(&path)
        .filter(|p| p != &shim_dir)
        .filter(|p| p.is_dir())
        .flat_map(|p| fs::read_dir(p).unwrap())
        .map(|bin| bin.unwrap().path())
        .flat_map(parse_command_version)
        .filter(|c| c.command_name == command)
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
        assert_eq!(
            res,
            Some(CommandVersion::new(
                "python",
                "2",
                Path::new("/usr/bin/python2"),
            ))
        )
    }

    #[test]
    fn command_with_version_suffix_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python2.7"));
        assert_eq!(
            res,
            Some(CommandVersion::new(
                "python",
                "2.7",
                Path::new("/usr/bin/python2.7"),
            ))
        )
    }

    #[test]
    fn command_with_version_suffix_and_dash_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/ruby-2.5"));
        assert_eq!(
            res,
            Some(CommandVersion::new(
                "ruby",
                "2.5",
                Path::new("/usr/bin/ruby-2.5"),
            ))
        )
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

    #[test]
    fn single_letter_command_with_single_digit_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/a2"));
        assert_eq!(
            res,
            Some(CommandVersion::new("a", "2", Path::new("/usr/bin/a2"),))
        );
    }

    #[test]
    fn single_letter_command_with_dot_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/a2.2"));
        assert_eq!(
            res,
            Some(CommandVersion::new("a", "2.2", Path::new("/usr/bin/a2.2"),))
        );
    }

    #[test]
    fn single_letter_command_with_dash_single_digit_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/a-2"));
        assert_eq!(
            res,
            Some(CommandVersion::new("a", "2", Path::new("/usr/bin/a-2"),))
        );
    }

    #[test]
    fn single_letter_command_with_dash_dot_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/a-2.2"));
        assert_eq!(
            res,
            Some(CommandVersion::new("a", "2.2", Path::new("/usr/bin/a-2.2"),))
        );
    }

    #[test]
    fn emoji_command_with_single_digit_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/💩2"));
        assert_eq!(
            res,
            Some(CommandVersion::new("💩", "2", Path::new("/usr/bin/💩2"),))
        );
    }

    #[test]
    fn emoji_command_with_dot_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/💩2.2"));
        assert_eq!(
            res,
            Some(CommandVersion::new(
                "💩",
                "2.2",
                Path::new("/usr/bin/💩2.2"),
            ))
        );
    }

    #[test]
    fn emoji_command_with_dash_single_digit_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/💩-2"));
        assert_eq!(
            res,
            Some(CommandVersion::new("💩", "2", Path::new("/usr/bin/💩-2"),))
        );
    }

    #[test]
    fn emoji_command_with_dash_dot_version() {
        let res = parse_command_version(PathBuf::from("/usr/bin/💩-2.2"));
        assert_eq!(
            res,
            Some(CommandVersion::new(
                "💩",
                "2.2",
                Path::new("/usr/bin/💩-2.2"),
            ))
        );
    }
}
