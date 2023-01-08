use std::{
    collections::HashMap,
    fmt::Display,
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

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AddError {
    #[error(
        "could not add {0} to CommandVersionRegistry because it's invalid: path should be absolute"
    )]
    InvalidPathNotAbsolute(CommandVersion),
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

impl Display for CommandVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0}@{1} ({2})",
            self.command_name,
            self.version_name,
            self.path.display()
        )
    }
}

type RegistryState = HashMap<String, HashMap<String, PathBuf>>;

#[derive(Debug)]
pub struct CommandVersionRegistry {
    path: PathBuf,
    state: RegistryState,
}

impl CommandVersionRegistry {
    pub fn load(path: &Path) -> Result<Self, io::Error> {
        let bytes = fs::read(path)?;
        let state: RegistryState = toml::from_slice(&bytes)?;

        Ok(CommandVersionRegistry {
            path: path.to_owned(),
            state,
        })
    }

    pub fn new(path: &Path) -> Self {
        CommandVersionRegistry {
            path: path.to_owned(),
            state: RegistryState::default(),
        }
    }

    pub fn load_or_new(path: &Path) -> Result<Self, io::Error> {
        let res = Self::load(path);
        res.or_else(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                Ok(Self::new(path))
            } else {
                Err(error)
            }
        })
    }

    pub fn save(&self) -> Result<(), SaveError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml = toml::to_string_pretty(&self.state)?;
        fs::write(&self.path, toml)?;
        Ok(())
    }

    pub fn get(&self, command: &str, version: &str) -> Option<CommandVersion> {
        let path = self.state.get(command)?.get(version).cloned()?;
        Some(CommandVersion::new(command, version, &path))
    }

    pub fn add(&mut self, command_version: CommandVersion) -> Result<(), AddError> {
        if !command_version.path.is_absolute() {
            return Err(AddError::InvalidPathNotAbsolute(command_version));
        }

        let command_entry = self.state.entry(command_version.command_name).or_default();
        command_entry.insert(command_version.version_name, command_version.path);

        Ok(())
    }

    pub fn remove(&mut self, command: &str, version: &str) {
        let versions = self.state.get_mut(command);
        if let Some(versions) = versions {
            versions.remove(version);
            if versions.is_empty() {
                self.state.remove(command);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = CommandVersion> + '_ {
        self.state.iter().flat_map(|(command_name, versions)| {
            versions
                .iter()
                .map(|(version_name, path)| CommandVersion::new(command_name, version_name, path))
        })
    }

    pub fn command_names(&self) -> impl Iterator<Item = String> + '_ {
        self.state.keys().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
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
        let registry = CommandVersionRegistry::new(Path::new("not-important"));
        assert!(registry.state.is_empty())
    }

    #[test]
    fn load_reads_empty_file() -> TestResult {
        let tmpfile = NamedTempFile::new()?;

        let registry = CommandVersionRegistry::load(tmpfile.path())?;

        assert_eq!(registry.state, RegistryState::default());

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
            registry.state,
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
    fn load_or_new_returns_default_on_missing_file() -> TestResult {
        let registry = CommandVersionRegistry::load_or_new(Path::new("does/not/exist"))?;

        assert_eq!(registry.state, RegistryState::default());

        Ok(())
    }

    #[test]
    fn load_or_new_crashes_with_io_error() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let bad_file_path = workdir.path().join("something");

        // The file path is actually a directory. This should cause an io::Error
        fs::create_dir(&bad_file_path)?;

        let res = CommandVersionRegistry::load_or_new(&bad_file_path);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn save_creates_new_file() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let file_path = workdir.path().join("registry.toml");

        let registry = CommandVersionRegistry {
            path: file_path.clone(),
            state: HashMap::from([(
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                    ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
                ]),
            )]),
        };

        assert!(!file_path.exists());

        registry.save()?;

        assert!(file_path.exists());
        assert!(file_path.is_file());

        Ok(())
    }

    #[test]
    fn save_creates_whole_path() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let parent_dir = workdir.path().join("some-dir");
        let file_path = parent_dir.join("registry.toml");

        let registry = CommandVersionRegistry {
            path: file_path.clone(),
            state: HashMap::from([(
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                    ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
                ]),
            )]),
        };

        assert!(!parent_dir.exists());
        assert!(!file_path.exists());

        registry.save()?;

        assert!(parent_dir.exists());
        assert!(parent_dir.is_dir());
        assert!(file_path.exists());
        assert!(file_path.is_file());

        Ok(())
    }

    #[test]
    fn save_and_load_preserves_data() -> TestResult {
        let tempfile = NamedTempFile::new()?;
        let new_registry = CommandVersionRegistry {
            path: tempfile.path().to_owned(),
            state: HashMap::from([
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
            ]),
        };

        new_registry.save()?;

        let loaded_registry = CommandVersionRegistry::load(tempfile.path())?;

        assert_eq!(new_registry.state, loaded_registry.state);

        Ok(())
    }

    #[test]
    fn add_creates_new_command() -> anyhow::Result<()> {
        let mut registry = CommandVersionRegistry::new(Path::new("not-important"));
        assert!(registry.state.is_empty());

        registry.add(CommandVersion::new(
            "the-command",
            "42",
            Path::new("/path/to/the-command-v42"),
        ))?;

        assert_eq!(
            registry.state,
            HashMap::from([(
                "the-command".to_string(),
                HashMap::from([("42".to_string(), PathBuf::from("/path/to/the-command-v42")),])
            )])
        );

        Ok(())
    }

    #[test]
    fn add_adds_version_to_existing_command() -> anyhow::Result<()> {
        let mut registry = CommandVersionRegistry::new(Path::new("not-important"));
        assert!(registry.state.is_empty());
        registry.add(CommandVersion::new(
            "the-command",
            "42",
            Path::new("/path/to/the-command-v42"),
        ))?;
        assert!(registry.state.contains_key("the-command"));

        registry.add(CommandVersion::new(
            "the-command",
            "43",
            Path::new("/path/to/the-command-v43"),
        ))?;
        assert_eq!(
            registry.state,
            HashMap::from([(
                "the-command".to_string(),
                HashMap::from([
                    ("42".to_string(), PathBuf::from("/path/to/the-command-v42")),
                    ("43".to_string(), PathBuf::from("/path/to/the-command-v43")),
                ])
            )])
        );

        Ok(())
    }

    #[test]
    fn add_fails_with_relative_path() {
        let mut registry = CommandVersionRegistry::new(Path::new("not-important"));

        let res = registry.add(CommandVersion::new(
            "foo",
            "42",
            Path::new("this/is/relative"),
        ));

        assert_eq!(
            res,
            Err(AddError::InvalidPathNotAbsolute(CommandVersion::new(
                "foo",
                "42",
                Path::new("this/is/relative")
            )))
        )
    }

    #[test]
    fn get_returns_path_when_found() {
        let registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([(
                "node".to_string(),
                HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
            )]),
        };

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
        let registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([(
                "node".to_string(),
                HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
            )]),
        };

        let res = registry.get("node", "not-there");
        assert_eq!(res, None);
    }

    #[test]
    fn get_returns_none_on_missing_command() {
        let registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([(
                "node".to_string(),
                HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
            )]),
        };

        let res = registry.get("not-there", "not-there");
        assert_eq!(res, None);
    }

    #[test]
    fn remove_does_nothing_when_version_does_not_exist() {
        let mut registry = CommandVersionRegistry::new(Path::new("not-important"));

        registry.remove("some-command", "42");

        assert_eq!(registry.state, RegistryState::default());
    }

    #[test]
    fn remove_removes_a_version_but_keeps_other_around() {
        let mut registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([
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
            ]),
        };

        registry.remove("foo", "42");

        assert_eq!(
            registry.state,
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
        let mut registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([
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
            ]),
        };

        registry.remove("foo", "43");

        assert_eq!(
            registry.state,
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
    fn iter_returns_no_results_on_empty() {
        let registry = CommandVersionRegistry::new(Path::new("not-important"));

        assert!(registry.iter().next().is_none());
    }

    #[test]
    fn iter_returns_everything() {
        let registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([
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
            ]),
        };

        let mut res = registry.iter().collect::<Vec<_>>();
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
    fn command_names_returns_nothing_with_empty_registry() {
        let registry = CommandVersionRegistry::new(Path::new("not-important"));

        assert!(registry.command_names().next().is_none());
    }

    #[test]
    fn command_names_returns_names_of_known_commands() {
        let registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([
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
            ]),
        };

        let mut res = registry.command_names().collect::<Vec<_>>();
        res.sort();

        assert_eq!(res, vec!["node", "the-command"]);
    }

    #[test]
    fn is_empty_returns_true_when_empty() {
        let registry = CommandVersionRegistry::new(Path::new("not-important"));
        assert!(registry.is_empty())
    }

    #[test]
    fn is_empty_returns_false_when_not_empty() {
        let registry = CommandVersionRegistry {
            path: PathBuf::from("not-important"),
            state: HashMap::from([(
                "node".to_string(),
                HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
            )]),
        };
        assert!(!registry.is_empty())
    }
}
