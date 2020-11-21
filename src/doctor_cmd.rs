use crate::def_file;
use crate::def_file::CommandDefs;
use std::collections::HashSet;
use std::path::Path;
use std::process;
use console;

pub fn run(fix: bool) {
    println!(
        "This command (alt doctor) will check for common / known problems and \
        will give some advice on how to fix them. Note that not all problems \
        are critical or require fixing. alt can still work perfectly fine \
        even with some problems."
    );
    println!();

    let mut problem_count: u32 = 0;
    let mut fixable_count: u32 = 0;
    let mut fixed_count: u32 = 0;

    let CheckDefsRes { defs, removed_defs } = check_defs(
        fix,
        &mut problem_count,
        &mut fixable_count,
        &mut fixed_count
    );

    if fix {
        def_file::save(&defs)
            .expect("failed to write command version definitions");
    }

    // TODO: check all used versions point to real versions
    // TODO: check that shims are defined
    // TODO: check that old shims are not left over

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

        if problem_count > fixed_count {
            process::exit(1);
        }
    } else {
        println!(
            "Found no problems. Everything should be fine. If you're still \
            experiencing issues, please file an issue: \
            https://github.com/dotboris/alt/issues/new"
        );
    }
}

struct CheckDefsRes {
    defs: CommandDefs,
    removed_defs: HashSet<(String, String)>
}
fn check_defs(
    fix: bool,
    problem_count: &mut u32,
    fixable_count: &mut u32,
    fixed_count: &mut u32
) -> CheckDefsRes {
    let defs = def_file::load();

    if defs.is_empty() {
        *problem_count += 1;
        print_problem(
            "No commands or command versions are defined. This is normal if \
            you've just installed alt. You will need to define some commands \
            & command versions in order to use alt. See: \
            https://github.com/dotboris/alt#define-command-versions"
        );
        println!();
    }

    let mut defs_to_remove = HashSet::new();

    for (command, versions) in defs {
        for (version, bin) in versions {
            let has_problem = def_has_problem(&command, &version, &bin);
            if has_problem {
                *problem_count += 1;

                defs_to_remove.insert(
                    (command.to_string(), version.to_string())
                );

                if fix {
                    *fixed_count += 1;
                    print_fixed(&format!(
                        "Removed entry for {} version {}.",
                        command, version
                    ));
                } else {
                    *fixable_count += 1;
                    print_fixable(&format!(
                        "Would remove entry for {} version {}.",
                        command, version
                    ));
                }
                println!();
            }
        }
    }

    let mut defs = def_file::load();
    for (command, version) in defs_to_remove.clone() {
        let versions = defs.get_mut(&command).unwrap();
        versions.remove(&version);
        if versions.is_empty() {
            defs.remove(&command);
        }
    }

    return CheckDefsRes {
        defs,
        removed_defs: defs_to_remove
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
