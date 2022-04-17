use std::path::*;

pub mod homebrew;
pub mod path_suffix;

#[derive(Debug, PartialEq)]
pub struct CommandVersion {
    pub command: String,
    pub version: String,
    pub path: PathBuf,
}
