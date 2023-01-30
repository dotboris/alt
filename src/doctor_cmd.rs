use crate::environment::load_command_version_registry;
use anyhow::Context;
use dialoguer::Confirm;
use std::os::unix::fs::MetadataExt;
use std::process;

pub enum FixMode {
    Auto,
    Never,
    Prompt,
}

pub fn run(fix_mode: FixMode) -> anyhow::Result<()> {
    let mut problem_count: u32 = 0;
    let mut fixed_count: u32 = 0;

    let mut command_version_registry = load_command_version_registry()?;

    for command_version in command_version_registry.iter().collect::<Vec<_>>() {
        let has_problem = {
            if !command_version.path.exists() {
                print_problem(&format!(
                    "Bin for {} version {} ({}) does not exist.",
                    command_version.command_name,
                    command_version.version_name,
                    command_version.path.display()
                ));
                true
            } else if !command_version.path.is_file() {
                print_problem(&format!(
                    "Bin for {} version {} ({}) is not a file.",
                    command_version.command_name,
                    command_version.version_name,
                    command_version.path.display()
                ));
                true
            } else if command_version.path.metadata().unwrap().mode() & 0o111 == 0 {
                print_problem(&format!(
                    "Bin for {} version {} ({}) is not executable.",
                    command_version.command_name,
                    command_version.version_name,
                    command_version.path.display()
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
                command_version.command_name, command_version.version_name,
            ));

            if should_fix(&fix_mode) {
                fixed_count += 1;

                command_version_registry
                    .remove(&command_version.command_name, &command_version.version_name);

                // We are purposefully saving at every step to ensure that
                // no fixes are lost if the user does a Ctrl-C to kill the
                // process.
                command_version_registry
                    .save()
                    .context("Failed to save command version definitions")?;

                print_fixed(&format!(
                    "Removed entry for {} version {}.",
                    command_version.command_name, command_version.version_name,
                ));
            }
            println!();
        }
    }

    if command_version_registry.is_empty() {
        problem_count += 1;
        print_problem(
            "No commands or command versions are defined. This is normal if \
            you've just installed alt. You will need to define some commands \
            & command versions in order to use alt. See: \
            https://github.com/dotboris/alt#define-command-versions",
        );
        println!();
    }

    // TODO: check all used versions point to real versions
    // TODO: check that shims are defined
    // TODO: check that old shims are not left over

    if problem_count > 0 {
        println!("Found {problem_count} problems. Fixed {fixed_count}/{problem_count}.");

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

    Ok(())
}

fn should_fix(fix_mode: &FixMode) -> bool {
    match fix_mode {
        FixMode::Auto => {
            println!("Applying fix because command was called with --fix-mode auto");
            true
        }
        FixMode::Never => {
            println!("Did not apply fix because command was called with --fix-mode never");
            false
        }
        FixMode::Prompt => Confirm::new()
            .with_prompt("Would you like to apply this fix?")
            .interact()
            .expect(
                "Failed to prompt for fix action. \
                    If you're trying to use this command non-interactively, \
                    try passing in --fix-mode <auto|never>",
            ),
    }
}

fn print_problem(message: &str) {
    println!("{}: {}", console::style("Problem").bold().yellow(), message);
}

fn print_fix_available(message: &str) {
    println!(
        "{}: {}",
        console::style("Fix Available").bold().cyan(),
        message
    );
}

fn print_fixed(message: &str) {
    println!("{}: {}", console::style("Fixed").bold().green(), message);
}
