extern crate toml;

use std::collections::HashMap;
use std::path::*;
use std::fs;
use std::io;
use crate::config;

const DEFS_FILE_NAME: &str = "defs.toml";

pub type CommandVersions = HashMap<String, PathBuf>;
pub type CommandDefs = HashMap<String, CommandVersions>;

pub fn load() -> CommandDefs {
    let path = config::home_dir().join(DEFS_FILE_NAME);
    if path.is_file() {
        let bytes = fs::read(path).expect("failed to read defs file");
        toml::from_slice(&bytes).expect("failed to parse defs file toml")
    } else {
        CommandDefs::new()
    }
}

pub fn save(defs: &CommandDefs) -> Result<(), io::Error> {
    let home_dir = config::home_dir();

    let toml = toml::to_string_pretty(defs)
        .expect("failed to serialize defs toml");
    fs::create_dir_all(&home_dir)?;
    fs::write(&home_dir.join(DEFS_FILE_NAME), toml)
}

pub fn find_bin<'a>(defs: &'a CommandDefs, command: &str, version: &str) -> Option<&'a PathBuf> {
    defs.get(command).and_then(|def| def.get(version))
}

pub fn remove_version(defs: &mut CommandDefs, command: &str, version: &str) {
    let versions = defs.get_mut(command);
    if let Some(versions) = versions {
        versions.remove(version);
        if versions.is_empty() {
            defs.remove(command);
        }
    }
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
        ruby.insert("2.4".to_string(), PathBuf::from("/path/to/ruby2.4/bin/ruby"));
        ruby.insert("2.5".to_string(), PathBuf::from("/path/to/ruby2.5/bin/ruby"));
        defs.insert("ruby".to_string(), ruby);

        let res = find_bin(&defs, "python", "2");

        assert_eq!(None, res)
    }

    #[test]
    fn find_bin_returns_none_when_version_not_defined() {
        let mut defs = CommandDefs::new();
        let mut ruby = CommandVersions::new();
        ruby.insert("2.4".to_string(), PathBuf::from("/path/to/ruby2.4/bin/ruby"));
        ruby.insert("2.5".to_string(), PathBuf::from("/path/to/ruby2.5/bin/ruby"));
        defs.insert("ruby".to_string(), ruby);

        let res = find_bin(&defs, "ruby", "1.9");

        assert_eq!(None, res)
    }

    #[test]
    fn find_bin_returns_path_when_command_and_version_defined() {
        let mut defs = CommandDefs::new();
        let mut ruby = CommandVersions::new();
        ruby.insert("2.4".to_string(), PathBuf::from("/path/to/ruby2.4/bin/ruby"));
        ruby.insert("2.5".to_string(), PathBuf::from("/path/to/ruby2.5/bin/ruby"));
        defs.insert("ruby".to_string(), ruby);

        let res = find_bin(&defs, "ruby", "2.4");

        assert_eq!(
            Some(&PathBuf::from("/path/to/ruby2.4/bin/ruby")),
            res
        )
    }

    #[test]
    fn remove_version_removes_given_command_version_pair() {
        let mut defs: CommandDefs = [
            ("node".to_string(),
                [
                    ("10".to_string(), PathBuf::from("/path/to/node10/bin/node")),
                    ("12".to_string(), PathBuf::from("/path/to/node12/bin/node")),
                ].iter().cloned().collect()
            ),
            ("python".to_string(),
                [
                    ("2.7".to_string(), PathBuf::from("/path/to/python-2.7/bin/python")),
                    ("3.9".to_string(), PathBuf::from("/path/to/python-3.9/bin/python")),
                ].iter().cloned().collect()
            ),
        ].iter().cloned().collect();

        let expected_defs: CommandDefs = [
            ("node".to_string(),
                [
                    ("12".to_string(), PathBuf::from("/path/to/node12/bin/node")),
                ].iter().cloned().collect()
            ),
            ("python".to_string(),
                [
                    ("2.7".to_string(), PathBuf::from("/path/to/python-2.7/bin/python")),
                    ("3.9".to_string(), PathBuf::from("/path/to/python-3.9/bin/python")),
                ].iter().cloned().collect()
            ),
        ].iter().cloned().collect();

        remove_version(&mut defs, "node", "10");
        assert_eq!(defs, expected_defs)
    }

    #[test]
    fn remove_version_removed_whole_command_def_when_left_empty() {
        let mut defs: CommandDefs = [
            ("node".to_string(),
                [
                    ("10".to_string(), PathBuf::from("/path/to/node10/bin/node")),
                    ("12".to_string(), PathBuf::from("/path/to/node12/bin/node")),
                ].iter().cloned().collect()
            ),
            ("python".to_string(),
                [
                    ("3.9".to_string(), PathBuf::from("/path/to/python-3.9/bin/python")),
                ].iter().cloned().collect()
            ),
        ].iter().cloned().collect();

        let expected_defs: CommandDefs = [
            ("node".to_string(),
                [
                    ("10".to_string(), PathBuf::from("/path/to/node10/bin/node")),
                    ("12".to_string(), PathBuf::from("/path/to/node12/bin/node")),
                ].iter().cloned().collect()
            ),
        ].iter().cloned().collect();

        remove_version(&mut defs, "python", "3.9");
        assert_eq!(defs, expected_defs)
    }

    #[test]
    fn remove_version_does_nothing_when_given_version_that_does_not_exist() {
        let mut defs: CommandDefs = [
            ("node".to_string(),
                [
                    ("10".to_string(), PathBuf::from("/path/to/node10/bin/node")),
                    ("12".to_string(), PathBuf::from("/path/to/node12/bin/node")),
                ].iter().cloned().collect()
            ),
            ("python".to_string(),
                [
                    ("2.7".to_string(), PathBuf::from("/path/to/python-2.7/bin/python")),
                    ("3.9".to_string(), PathBuf::from("/path/to/python-3.9/bin/python")),
                ].iter().cloned().collect()
            ),
        ].iter().cloned().collect();

        let expected_defs = defs.clone();

        remove_version(&mut defs, "ruby", "1.9");
        assert_eq!(defs, expected_defs)
    }

    #[test]
    fn remove_version_does_nothing_on_empty_defs() {
        let mut defs = CommandDefs::new();
        remove_version(&mut defs, "something", "something");
        assert_eq!(
            defs,
            CommandDefs::new()
        )
    }
}
