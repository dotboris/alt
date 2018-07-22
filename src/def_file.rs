extern crate toml;

use std::collections::HashMap;
use std::fs;
use config;

const DEFS_FILE_NAME: &'static str = "defs.toml";

pub type CommandVersions = HashMap<String, String>;
pub type CommandDefs = HashMap<String, CommandVersions>;

pub fn load() -> CommandDefs {
    let path = config::home_dir().join(DEFS_FILE_NAME);
    if path.is_file() {
        let bytes = fs::read(path).expect("failed to read defs file");
        toml::from_slice(&bytes).expect("failted to parse defs file toml")
    } else {
        CommandDefs::new()
    }
}

pub fn find_bin<'a>(defs: &'a CommandDefs, command: &str, version: &str) -> Option<&'a String> {
    defs.get(command).and_then(|def| def.get(version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_bin_returns_none_on_empty_defs() {
        let defs = CommandDefs::new();

        let res = find_bin(&defs, "python", "2");

        assert_eq!(None, res)
    }

    #[test]
    fn find_bin_returns_none_when_command_not_defined() {
        let mut defs = CommandDefs::new();
        let mut ruby = CommandVersions::new();
        ruby.insert("2.4".to_string(), "/path/to/ruby2.4/bin/ruby".to_string());
        ruby.insert("2.5".to_string(), "/path/to/ruby2.5/bin/ruby".to_string());
        defs.insert("ruby".to_string(), ruby);

        let res = find_bin(&defs, "python", "2");

        assert_eq!(None, res)
    }

    #[test]
    fn find_bin_returns_none_when_version_not_defined() {
        let mut defs = CommandDefs::new();
        let mut ruby = CommandVersions::new();
        ruby.insert("2.4".to_string(), "/path/to/ruby2.4/bin/ruby".to_string());
        ruby.insert("2.5".to_string(), "/path/to/ruby2.5/bin/ruby".to_string());
        defs.insert("ruby".to_string(), ruby);

        let res = find_bin(&defs, "ruby", "1.9");

        assert_eq!(None, res)
    }

    #[test]
    fn find_bin_returns_path_when_command_and_version_defined() {
        let mut defs = CommandDefs::new();
        let mut ruby = CommandVersions::new();
        ruby.insert("2.4".to_string(), "/path/to/ruby2.4/bin/ruby".to_string());
        ruby.insert("2.5".to_string(), "/path/to/ruby2.5/bin/ruby".to_string());
        defs.insert("ruby".to_string(), ruby);

        let res = find_bin(&defs, "ruby", "2.4");

        assert_eq!(
            Some(&"/path/to/ruby2.4/bin/ruby".to_string()),
            res
        )
    }
}
