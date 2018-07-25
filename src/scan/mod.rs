use std::path::*;

pub mod path_suffix;
pub mod homebrew;

#[derive(Debug, PartialEq)]
pub struct CommandVersion {
    pub command: String,
    pub version: String,
    pub path: PathBuf,
}
