use config;
use std::env;
use std::iter;
use console;
use console::style;

fn line(width: usize) -> String {
    let line: String = iter::repeat("=").take(width).collect();
    return format!("{}", style(line).bold());
}

fn line_label(width: usize, label: &str) -> String {
    let label_size = label.len() + 4;
    let left_size = (width - label_size) / 2;
    let right_size = width - left_size - label_size;

    let left: String = iter::repeat("=").take(left_size).collect();
    let right: String = iter::repeat("=").take(right_size).collect();

    return format!(
        "{} {} {}",
        style(format!("{}>", left)).bold(),
        style(label).bold().red(),
        style(format!("<{}", right)).bold(),
    );
}

pub fn check_shim_in_path() {
    let path = env::var("PATH").unwrap();
    let mut parts = env::split_paths(&path);

    let shim_dir = config::shim_dir();
    if !parts.any(|part| part == shim_dir) {
        let term = console::Term::stdout();
        let (_, term_width) = term.size();

        let shim_dir = shim_dir.to_str().unwrap();
        let shim_dir = style(shim_dir).cyan();
        let path_env_var = style("PATH").cyan();

        eprintln!("{}", line_label(term_width as usize, "WARNING"));
        eprintln!("Alt is not installed corrected and will not work!");
        eprintln!();
        eprintln!(
            "You are seeing the warning because the shim directory ({}) is not in your {} environment variable.",
            shim_dir,
            path_env_var
        );
        eprintln!();
        eprintln!("Please add {} to your {} environment variable.", shim_dir, path_env_var);
        eprintln!();
        eprintln!(
            "Alternatively, see {} for setup instructions.",
            style("https://github.com/dotboris/alt#installation").cyan()
        );
        eprintln!("{}", line(term_width as usize));
        eprintln!();
    }
}
