use crate::config;
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
    let path = env::var("PATH").unwrap_or_default();
    let mut parts = env::split_paths(&path);

    let shim_dir = config::shim_dir();
    if !parts.any(|part| part == shim_dir) {
        let term = console::Term::stdout();
        let (_, term_width) = term.size();

        let shim_dir = shim_dir.to_str().unwrap();
        let shim_dir = style(shim_dir).cyan();
        let path_env_var = style("PATH").cyan();

        eprintln!(
            "\
{warning_line}
Alt is not configured corrected and will not work!

You are seeing the warning because the shim directory ({shim_dir}) is not in \
your {path_env_var} environment variable.

Normally, alt should configure this automatically during the install process.
In come cases you may need to:

- {reopen_terminal}
- {relogin}

If the problem persists, please see:
    {troubleshooting_link}
{bottom_line}

",
            warning_line=line_label(term_width as usize, "WARNING"),
            bottom_line=line(term_width as usize),
            shim_dir=shim_dir,
            path_env_var=path_env_var,
            reopen_terminal=style("re-open your terminal").bold(),
            relogin=style("log out and log back into your user session").bold(),
            troubleshooting_link=style("https://github.com/dotboris/alt#troubleshooting").cyan()
        );
    }
}
