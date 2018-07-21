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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_alt_command_is_shim() {
        assert_eq!(true, is_shim("node"))
    }

    #[test]
    fn test_absolute_non_alt_is_not_shim() {
        assert_eq!(true, is_shim("/home/whatever/.local/alt/shims/node"))
    }

    #[test]
    fn test_relative_non_alt_is_not_shim() {
        assert_eq!(true, is_shim("./my/node"))
    }

    #[test]
    fn test_alt_command_is_not_shim() {
        assert_eq!(false, is_shim("alt"))
    }

    #[test]
    fn test_absolute_alt_is_not_shim() {
        assert_eq!(false, is_shim("/usr/bin/alt"))
    }

    #[test]
    fn test_relative_alt_is_not_shim() {
        assert_eq!(false, is_shim("./target/debug/alt"))
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
