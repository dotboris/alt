use crate::def_file;
use std::path::Path;
use console;

pub fn run(fix: bool) {
    println!(
        "This command (alt doctor) will check for common / known problems and \
        will give some advice on how to fix them. Note that not all problems \
        are critical or require fixing. alt can still work perfectly fine \
        even with some problems."
    );
    println!();

    let mut problem_count = 0;
    let mut fixable_count = 0;
    let mut fixed_count = 0;

    let defs = def_file::load();

    if defs.is_empty() {
        problem_count += 1;
        print_problem(
            "No commands or command versions are defined. This is normal if \
            you've just installed alt. You will need to define some commands \
            & command versions in order to use alt. See: \
            https://github.com/dotboris/alt#define-command-versions"
        );
        println!();
    }

    for (command, versions) in defs {
        for (version, bin) in versions {
            let has_problem = def_has_problem(&command, &version, &bin);
            if has_problem {
                problem_count += 1;
                if fix {
                    fixed_count += 1;
                    // TODO: fix
                    print_fixed(&format!(
                        "Removed entry for {} version {}.",
                        command, version
                    ));
                } else {
                    fixable_count += 1;
                    print_fixable(&format!(
                        "Would remove entry for {} version {}.",
                        command, version
                    ));
                }
                println!();
            }
        }
    }

    // TODO: check all defs point to real files
    // TODO: check all used versions point to real versions
    // TODO: check that shims are defined

    if problem_count > 0 {
        if fix {
            println!(
                "Found {} problems. {} were automatically fixed.",
                problem_count,
                fixed_count
            );
        } else {
            if fixable_count > 0 {
                println!(
                    "Found {} problems. {} can automatically be fixed. \
                    Run alt doctor --fix to automatically fix these problems.",
                    problem_count,
                    fixable_count
                );
            } else {
                println!(
                    "Found {} problems. None can be automatically fixed.",
                    problem_count
                );
            }
        }
    } else {
        println!(
            "Found no problems. Everything should be fine. If you're still \
            experiencing issues, please file an issue: \
            https://github.com/dotboris/alt/issues/new"
        );
    }
}

fn def_has_problem(command: &str, version: &str, bin: &Path) -> bool {
    if !bin.exists() {
        print_problem(&format!(
            "Binary for {} version {} ({}) does not exist.",
            command, version, bin.display()
        ));
        return true;
    }

    if !bin.is_file() {
        print_problem(&format!(
            "Binary for {} version {} ({}) is not a file.",
            command, version, bin.display()
        ));
        return true;
    }

    // TODO: check permissions (readable + executable)

    return false;
}

fn print_problem(message: &str) {
    println!("{}: {}",
        console::style("Problem").bold().yellow(),
        message
    );
}

fn print_fixable(message: &str) {
    println!("{}: {}",
        console::style("Auto-Fixable").bold().cyan(),
        message
    );
}

fn print_fixed(message: &str) {
    println!("{}: {}",
        console::style("Fixed").bold().green(),
        message
    );
}
