use config;
use std::env;
use console::style;

pub fn check_shim_in_path() {
    let path = env::var("PATH").unwrap();
    let mut parts = env::split_paths(&path);

    let shim_dir = config::shim_dir();
    if !parts.any(|part| part == shim_dir) {
        eprintln!(
            "{}: Shims are not in your {}. Alt will not function properly.",
            style("WARNING").bold().yellow(),
            style("PATH").bold().cyan()
        );
        eprintln!(
            "See {} for setup instructions.",
            style("https://github.com/dotboris/alt#installation").cyan()
        );
    }
}
