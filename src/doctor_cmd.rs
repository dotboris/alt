use crate::def_file;
use std::process;
use std::os::unix::fs::MetadataExt;
use dialoguer::Confirm;

pub enum FixMode { Auto, Never, Prompt }

pub fn run(fix_mode: FixMode) {
    let mut problem_count: u32 = 0;
    let mut fixed_count: u32 = 0;

    let defs = def_file::load();
    let mut fixed_defs = defs.clone();

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
            let has_problem = {
                if !bin.exists() {
                    print_problem(&format!(
                        "Bin for {} version {} ({}) does not exist.",
                        command, version, bin.display()
                    ));
                    true
                } else if !bin.is_file() {
                    print_problem(&format!(
                        "Bin for {} version {} ({}) is not a file.",
                        command, version, bin.display()
                    ));
                    true
                } else if bin.metadata().unwrap().mode() & 0o111 == 0 {
                    print_problem(&format!(
                        "Bin for {} version {} ({}) is not executable.",
                        command, version, bin.display()
                    ));
                    true
                } else {
                    false
                }
            };

            if has_problem {
                problem_count += 1;

                print_fix_available(&format!(
                    "Remove entry for {} version {}.",
                    command, version
                ));

                if should_fix(&fix_mode) {
                    fixed_count += 1;

                    def_file::remove_version(&mut fixed_defs, &command, &version);

                    // We are purposefully saving at every step to ensure that
                    // no fixes are lost if the user does a Ctrl-C to kill the
                    // process.
                    def_file::save(&fixed_defs)
                        .expect("Failed to save command version definitions");

                    print_fixed(&format!(
                        "Removed entry for {} version {}.",
                        command, version
                    ));
                }
                println!();
            }
        }
    }

    // TODO: check all used versions point to real versions
    // TODO: check that shims are defined
    // TODO: check that old shims are not left over

    if problem_count > 0 {
        println!(
            "Found {} problems. Fixed {}/{}.",
            problem_count,
            fixed_count,
            problem_count
        );

        if problem_count > fixed_count {
            process::exit(1);
        }
    } else {
        println!(
            "{}: Found no problems. Alt should be working correctly. If you're \
            still experiencing problems, please open an issue: \
            https://github.com/dotboris/alt/issues/new",
            console::style("All is good").bold().green()
        );
    }
}

fn should_fix(fix_mode: &FixMode) -> bool {
    match fix_mode {
        FixMode::Auto => {
            println!("Applying fix because command was called with --fix-mode auto");
            true
        },
        FixMode::Never => {
            println!("Did not apply fix because command was called with --fix-mode never");
            false
        },
        FixMode::Prompt => {
            Confirm::new()
                .with_prompt("Would you like to apply this fix?")
                .interact()
                .expect(
                    "Failed to prompt for fix action. \
                    If you're trying to use this command non-interactively, \
                    try passing in --fix-mod <auto|never>"
                )
        },
    }
}

fn print_problem(message: &str) {
    println!("{}: {}",
        console::style("Problem").bold().yellow(),
        message
    );
}

fn print_fix_available(message: &str) {
    println!("{}: {}",
        console::style("Fix Available").bold().cyan(),
        message
    );
}

fn print_fixed(message: &str) {
    println!("{}: {}",
        console::style("Fixed").bold().green(),
        message
    );
}
