use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(thiserror::Error, Debug)]
pub enum SaveError {
    #[error("failed to serialize CommandVersionRegistry contents as TOML")]
    TomlError(#[from] toml::ser::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct CommandVersion {
    pub command_name: String,
    pub version_name: String,
    pub path: PathBuf,
}

impl CommandVersion {
    pub fn new(command_name: &str, version_name: &str, path: &Path) -> Self {
        CommandVersion {
            command_name: command_name.to_owned(),
            version_name: version_name.to_owned(),
            path: path.to_owned(),
        }
    }
}

type RegistryState = HashMap<String, HashMap<String, PathBuf>>;

#[derive(Debug, PartialEq, Default)]
pub struct CommandVersionRegistry(RegistryState);

impl CommandVersionRegistry {
    pub fn load(path: &Path) -> Result<Self, io::Error> {
        let bytes = fs::read(path)?;
        let state: RegistryState = toml::from_slice(&bytes)?;

        Ok(CommandVersionRegistry(state))
    }

    pub fn load_or_default(path: &Path) -> Result<Self, io::Error> {
        let res = Self::load(path);
        res.or_else(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                Ok(Default::default())
            } else {
                Err(error)
            }
        })
    }

    pub fn save(&self, path: &Path) -> Result<(), SaveError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml = toml::to_string_pretty(&self.0)?;
        fs::write(path, toml)?;
        Ok(())
    }

    pub fn get(&self, command: &str, version: &str) -> Option<CommandVersion> {
        let path = self.0.get(command)?.get(version).cloned()?;
        Some(CommandVersion::new(command, version, &path))
    }

    pub fn add(&mut self, command_version: CommandVersion) {
        let command_entry = self.0.entry(command_version.command_name).or_default();
        command_entry.insert(command_version.version_name, command_version.path);
    }

    pub fn remove(&mut self, command: &str, version: &str) {
        let versions = self.0.get_mut(command);
        if let Some(versions) = versions {
            versions.remove(version);
            if versions.is_empty() {
                self.0.remove(command);
            }
        }
    }

    pub fn all(&self) -> impl Iterator<Item = CommandVersion> + '_ {
        self.0.iter().flat_map(|(command_name, versions)| {
            versions
                .iter()
                .map(|(version_name, path)| CommandVersion::new(command_name, version_name, path))
        })
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use tempfile::{self, NamedTempFile};

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn default_is_empty() {
        let registry: CommandVersionRegistry = Default::default();
        assert!(registry.0.is_empty())
    }

    #[test]
    fn load_reads_empty_file() -> TestResult {
        let tmpfile = NamedTempFile::new()?;

        let registry = CommandVersionRegistry::load(tmpfile.path())?;

        assert_eq!(registry.0, RegistryState::default());

        Ok(())
    }

    #[test]
    fn load_reads_file_with_content() -> TestResult {
        let tmpfile = NamedTempFile::new()?;
        fs::write(
            tmpfile.path(),
            concat!(
                "[the-command]\n",
                "\"42\" = \"/path/to/something\"\n",
                "\"43\" = \"/path/to/something-else\"\n",
            ),
        )?;

        let registry = CommandVersionRegistry::load(tmpfile.path())?;

        assert_eq!(
            registry.0,
            HashMap::from([(
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("/path/to/something")),
                    ("43".to_string(), PathBuf::from("/path/to/something-else")),
                ])
            )])
        );

        Ok(())
    }

    #[test]
    fn load_crashes_on_missing_file() {
        let res = CommandVersionRegistry::load(Path::new("does/not/exist"));
        assert!(res.is_err())
    }

    #[test]
    fn load_or_default_returns_default_on_missing_file() -> TestResult {
        let registry = CommandVersionRegistry::load_or_default(Path::new("does/not/exist"))?;

        assert_eq!(registry, CommandVersionRegistry::default());

        Ok(())
    }

    #[test]
    fn load_or_default_crashes_with_io_error() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let bad_file_path = workdir.path().join("something");

        // The file path is actually a directory. This should cause an io::Error
        fs::create_dir(&bad_file_path)?;

        let res = CommandVersionRegistry::load_or_default(&bad_file_path);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn save_creates_new_file() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let file_path = workdir.path().join("registry.toml");

        let registry = CommandVersionRegistry(HashMap::from([(
            "the-command".to_string(),
            HashMap::from([
                ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
            ]),
        )]));

        assert!(!file_path.exists());

        registry.save(&file_path)?;

        assert!(file_path.exists());
        assert!(file_path.is_file());

        Ok(())
    }

    #[test]
    fn save_creates_whole_path() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let parent_dir = workdir.path().join("some-dir");
        let file_path = parent_dir.join("registry.toml");

        let registry = CommandVersionRegistry(HashMap::from([(
            "the-command".to_string(),
            HashMap::from([
                ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
            ]),
        )]));

        assert!(!parent_dir.exists());
        assert!(!file_path.exists());

        registry.save(&file_path)?;

        assert!(parent_dir.exists());
        assert!(parent_dir.is_dir());
        assert!(file_path.exists());
        assert!(file_path.is_file());

        Ok(())
    }

    #[test]
    fn save_and_load_preserves_data() -> TestResult {
        let tempfile = NamedTempFile::new()?;
        let new_registry = CommandVersionRegistry(HashMap::from([
            (
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                    ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
                ]),
            ),
            (
                "node".to_string(),
                HashMap::from([
                    ("16".to_string(), PathBuf::from("path/to/node-16")),
                    ("18".to_string(), PathBuf::from("path/to/node-18")),
                ]),
            ),
        ]));

        new_registry.save(tempfile.path())?;

        let loaded_registry = CommandVersionRegistry::load(tempfile.path())?;

        assert_eq!(new_registry, loaded_registry);

        Ok(())
    }

    #[test]
    fn add_creates_new_command() {
        let mut registry = CommandVersionRegistry::default();
        assert!(registry.0.is_empty());

        registry.add(CommandVersion::new(
            "the-command",
            "42",
            Path::new("path/to/the-command-v42"),
        ));

        assert_eq!(
            registry.0,
            HashMap::from([(
                "the-command".to_string(),
                HashMap::from([("42".to_string(), PathBuf::from("path/to/the-command-v42")),])
            )])
        );
    }

    #[test]
    fn add_adds_version_to_existing_command() {
        let mut registry = CommandVersionRegistry::default();
        assert!(registry.0.is_empty());
        registry.add(CommandVersion::new(
            "the-command",
            "42",
            Path::new("path/to/the-command-v42"),
        ));
        assert!(registry.0.contains_key("the-command"));

        registry.add(CommandVersion::new(
            "the-command",
            "43",
            Path::new("path/to/the-command-v43"),
        ));
        assert_eq!(
            registry.0,
            HashMap::from([(
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                    ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
                ])
            )])
        );
    }

    #[test]
    fn get_returns_path_when_found() {
        let registry = CommandVersionRegistry(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));

        let res = registry.get("node", "18");
        assert_eq!(
            res,
            Some(CommandVersion {
                command_name: "node".to_string(),
                version_name: "18".to_owned(),
                path: PathBuf::from("path/to/node-18")
            })
        );
    }

    #[test]
    fn get_returns_none_on_missing_version() {
        let registry = CommandVersionRegistry(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));

        let res = registry.get("node", "not-there");
        assert_eq!(res, None);
    }

    #[test]
    fn get_returns_none_on_missing_command() {
        let registry = CommandVersionRegistry(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));

        let res = registry.get("not-there", "not-there");
        assert_eq!(res, None);
    }

    #[test]
    fn remove_does_nothing_when_version_does_not_exist() {
        let mut registry = CommandVersionRegistry::default();

        registry.remove("some-command", "42");

        assert_eq!(registry.0, RegistryState::default());
    }

    #[test]
    fn remove_removes_a_version_but_keeps_other_around() {
        let mut registry = CommandVersionRegistry(HashMap::from([
            (
                "foo".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("path/to/foo-42")),
                    ("43".to_string(), PathBuf::from("path/to/foo-43")),
                ]),
            ),
            (
                "node".to_string(),
                HashMap::from([
                    ("16".to_string(), PathBuf::from("path/to/node-16")),
                    ("18".to_string(), PathBuf::from("path/to/node-18")),
                ]),
            ),
        ]));

        registry.remove("foo", "42");

        assert_eq!(
            registry.0,
            HashMap::from([
                (
                    "foo".to_string(),
                    HashMap::from([("43".to_string(), PathBuf::from("path/to/foo-43")),]),
                ),
                (
                    "node".to_string(),
                    HashMap::from([
                        ("16".to_string(), PathBuf::from("path/to/node-16")),
                        ("18".to_string(), PathBuf::from("path/to/node-18")),
                    ]),
                ),
            ])
        );
    }

    #[test]
    fn remove_cleans_up_command_hashmap_when_removing_last_version() {
        let mut registry = CommandVersionRegistry(HashMap::from([
            (
                "foo".to_string(),
                HashMap::from([("43".to_string(), PathBuf::from("path/to/foo-43"))]),
            ),
            (
                "node".to_string(),
                HashMap::from([
                    ("16".to_string(), PathBuf::from("path/to/node-16")),
                    ("18".to_string(), PathBuf::from("path/to/node-18")),
                ]),
            ),
        ]));

        registry.remove("foo", "43");

        assert_eq!(
            registry.0,
            HashMap::from([(
                "node".to_string(),
                HashMap::from([
                    ("16".to_string(), PathBuf::from("path/to/node-16")),
                    ("18".to_string(), PathBuf::from("path/to/node-18")),
                ]),
            ),])
        );
    }

    #[test]
    fn all_returns_no_results_on_empty() {
        let registry = CommandVersionRegistry::default();

        assert!(registry.all().next().is_none());
    }

    #[test]
    fn all_returns_everything() {
        let registry = CommandVersionRegistry(HashMap::from([
            (
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                    ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
                ]),
            ),
            (
                "node".to_string(),
                HashMap::from([
                    ("16".to_string(), PathBuf::from("path/to/node-16")),
                    ("18".to_string(), PathBuf::from("path/to/node-18")),
                ]),
            ),
        ]));

        let mut res = registry.all().collect::<Vec<_>>();
        res.sort_by(|a, b| {
            (&a.command_name, &a.version_name).cmp(&(&b.command_name, &b.version_name))
        });

        assert_eq!(
            res,
            vec![
                CommandVersion::new("node", "16", Path::new("path/to/node-16")),
                CommandVersion::new("node", "18", Path::new("path/to/node-18")),
                CommandVersion::new("the-command", "42", Path::new("path/to/the-command-v42")),
                CommandVersion::new("the-command", "43", Path::new("path/to/the-command-v43")),
            ]
        )
    }

    #[test]
    fn is_empty_returns_true_when_empty() {
        let registry = CommandVersionRegistry::default();
        assert!(registry.is_empty())
    }

    #[test]
    fn is_empty_returns_false_when_not_empty() {
        let registry = CommandVersionRegistry(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));
        assert!(!registry.is_empty())
    }
}
