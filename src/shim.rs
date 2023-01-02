use crate::environment;
use std::fs;
use std::io;
use std::os::unix::fs as unix_fs;
use std::path::Path;

pub fn is_shim(arg0: &str) -> bool {
    get_command(arg0) != "alt"
}

pub fn get_command(arg0: &str) -> &str {
    Path::new(arg0)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap()
}

pub fn empty_shim_dir() -> Result<(), io::Error> {
    let root = environment::shim_dir();

    if root.is_dir() {
        fs::remove_dir_all(&root)?;
    }

    fs::create_dir_all(&root)
}

pub fn make_shim(command: &str, exe: &Path) -> Result<(), io::Error> {
    let root = environment::shim_dir();
    fs::create_dir_all(&root)?;
    let link = root.join(command);
    if link.exists() {
        fs::remove_file(&link)?;
    }
    unix_fs::symlink(exe, &link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_alt_command_is_shim() {
        assert!(is_shim("node"))
    }

    #[test]
    fn test_absolute_non_alt_is_not_shim() {
        assert!(is_shim("/home/whatever/.local/alt/shims/node"))
    }

    #[test]
    fn test_relative_non_alt_is_not_shim() {
        assert!(is_shim("./my/node"))
    }

    #[test]
    fn test_alt_command_is_not_shim() {
        assert!(!is_shim("alt"))
    }

    #[test]
    fn test_absolute_alt_is_not_shim() {
        assert!(!is_shim("/usr/bin/alt"))
    }

    #[test]
    fn test_relative_alt_is_not_shim() {
        assert!(!is_shim("./target/debug/alt"))
    }

    #[test]
    fn test_get_command_command() {
        assert_eq!("foo", get_command("foo"))
    }

    #[test]
    fn test_get_command_relative() {
        assert_eq!("foo", get_command("./something/foo"))
    }

    #[test]
    fn test_get_command_absolute() {
        assert_eq!("foo", get_command("/usr/bin/foo"))
    }
}
