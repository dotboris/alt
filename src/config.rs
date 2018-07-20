use std::path::{Path, PathBuf};
use std::env;

const DEFAULT_HOME: &'static str = ".config/alt";

pub fn home_dir() -> PathBuf {
    match env::var("ALT_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(DEFAULT_HOME)
        },
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use config;
    use std::path::{Path, PathBuf};

    #[test]
    fn home_dir_should_default() {
        env::remove_var("ALT_HOME");
        assert_eq!(
            config::home_dir(),
            Path::new(&env::var("HOME").unwrap())
                .join(config::DEFAULT_HOME)
        )
    }

    #[test]
    fn home_dir_should_read_alt_home_env() {
        env::set_var("ALT_HOME", "/path/to/phony/home");
        assert_eq!(
            config::home_dir(),
            PathBuf::from("/path/to/phony/home")
        )
    }
}
