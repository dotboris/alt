use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

type DefinitionsState = HashMap<String, HashMap<String, PathBuf>>;

#[derive(thiserror::Error, Debug)]
pub enum SaveError {
    #[error("failed to serialize definitions as TOML")]
    TomlError(#[from] toml::ser::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, PartialEq, Default)]
pub struct Definitions(DefinitionsState);

impl Definitions {
    pub fn load(path: &Path) -> Result<Self, io::Error> {
        let bytes = fs::read(path)?;
        let state: DefinitionsState = toml::from_slice(&bytes)?;

        Ok(Definitions(state))
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

    pub fn add_version(&mut self, command: &str, version: &str, binary_path: &Path) {
        let command_entry = self.0.entry(command.to_string()).or_default();
        command_entry.insert(version.to_string(), binary_path.to_owned());
    }

    pub fn get_binary(&self, command: &str, version: &str) -> Option<PathBuf> {
        self.0.get(command)?.get(version).cloned()
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
        let definitions: Definitions = Default::default();
        assert!(definitions.0.is_empty())
    }

    #[test]
    fn load_reads_empty_file() -> TestResult {
        let tmpfile = NamedTempFile::new()?;

        let definitions = Definitions::load(tmpfile.path())?;

        assert_eq!(definitions.0, DefinitionsState::default());

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

        let definitions = Definitions::load(tmpfile.path())?;

        assert_eq!(
            definitions.0,
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
        let res = Definitions::load(Path::new("does/not/exist"));
        assert!(res.is_err())
    }

    #[test]
    fn load_or_default_returns_default_on_missing_file() -> TestResult {
        let definitions = Definitions::load_or_default(Path::new("does/not/exist"))?;

        assert_eq!(definitions, Definitions::default());

        Ok(())
    }

    #[test]
    fn load_or_default_crashes_with_io_error() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let bad_file_path = workdir.path().join("something");

        // The file path is actually a directory. This should cause an io::Error
        fs::create_dir(&bad_file_path)?;

        let res = Definitions::load_or_default(&bad_file_path);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn save_creates_new_file() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let file_path = workdir.path().join("definitions.toml");

        let definitions = Definitions(HashMap::from([(
            "the-command".to_string(),
            HashMap::from([
                ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
            ]),
        )]));

        assert!(!file_path.exists());

        definitions.save(&file_path)?;

        assert!(file_path.exists());
        assert!(file_path.is_file());

        Ok(())
    }

    #[test]
    fn save_creates_whole_path() -> TestResult {
        let workdir = tempfile::tempdir()?;
        let parent_dir = workdir.path().join("some-dir");
        let file_path = parent_dir.join("definitions.toml");

        let definitions = Definitions(HashMap::from([(
            "the-command".to_string(),
            HashMap::from([
                ("42".to_string(), PathBuf::from("path/to/the-command-v42")),
                ("43".to_string(), PathBuf::from("path/to/the-command-v43")),
            ]),
        )]));

        assert!(!parent_dir.exists());
        assert!(!file_path.exists());

        definitions.save(&file_path)?;

        assert!(parent_dir.exists());
        assert!(parent_dir.is_dir());
        assert!(file_path.exists());
        assert!(file_path.is_file());

        Ok(())
    }

    #[test]
    fn save_and_load_preserves_data() -> TestResult {
        let tempfile = NamedTempFile::new()?;
        let definitions = Definitions(HashMap::from([
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

        definitions.save(tempfile.path())?;

        let loaded_definitions = Definitions::load(tempfile.path())?;

        assert_eq!(definitions, loaded_definitions);

        Ok(())
    }

    #[test]
    fn add_version_creates_new_command() {
        let mut definitions = Definitions::default();
        assert!(definitions.0.is_empty());

        definitions.add_version("the-command", "42", Path::new("path/to/the-command-v42"));

        assert_eq!(
            definitions.0,
            HashMap::from([(
                "the-command".to_string(),
                HashMap::from([("42".to_string(), PathBuf::from("path/to/the-command-v42")),])
            )])
        );
    }

    #[test]
    fn add_version_adds_version_to_existing_command() {
        let mut definitions = Definitions::default();
        assert!(definitions.0.is_empty());
        definitions.add_version("the-command", "42", Path::new("path/to/the-command-v42"));
        assert!(definitions.0.contains_key("the-command"));

        definitions.add_version("the-command", "43", Path::new("path/to/the-command-v43"));
        assert_eq!(
            definitions.0,
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
    fn get_binary_returns_path_when_found() {
        let definitions = Definitions(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));

        let res = definitions.get_binary("node", "18");
        assert_eq!(res, Some(PathBuf::from("path/to/node-18")));
    }

    #[test]
    fn get_binary_returns_none_on_missing_version() {
        let definitions = Definitions(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));

        let res = definitions.get_binary("node", "not-there");
        assert_eq!(res, None);
    }

    #[test]
    fn get_binary_returns_none_on_missing_command() {
        let definitions = Definitions(HashMap::from([(
            "node".to_string(),
            HashMap::from([("18".to_string(), PathBuf::from("path/to/node-18"))]),
        )]));

        let res = definitions.get_binary("not-there", "not-there");
        assert_eq!(res, None);
    }
}
