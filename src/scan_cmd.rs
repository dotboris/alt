use crate::command_version::CommandVersion;
use crate::environment::load_command_version_registry;
use crate::scan;
use crate::shim;
use anyhow::Context;
use dialoguer::MultiSelect;
use std::env;
use std::process;

fn prompt_versions(versions: &[CommandVersion]) -> Vec<usize> {
    let items: Vec<_> = versions
        .iter()
        .map(|version| {
            format!(
                "{} {} ({})",
                version.command_name,
                version.version_name,
                version.path.to_str().unwrap()
            )
        })
        .collect();

    println!("Here are the versions I found.");
    println!("  ↑/↓,j/k: move cursor");
    println!("  <space>: toggle keep");
    println!("  <enter>: confirm");
    println!();

    MultiSelect::new()
        .items(items.as_slice())
        .clear(false)
        .interact()
        .unwrap()
}

pub fn run(command: &str) -> anyhow::Result<()> {
    let versions: Vec<_> = scan::path_suffix::scan(command)
        .into_iter()
        .chain(scan::homebrew::scan(command))
        .collect();

    if versions.is_empty() {
        println!("Sorry, could not find any versions of {command}");
        process::exit(1);
    } else {
        let choices = prompt_versions(&versions);

        if choices.is_empty() {
            println!("Looks like you didn't choose anything.");
            println!("Did you forget to select versions with <space>?");
        } else {
            let mut command_version_registry = load_command_version_registry()?;

            for choice in choices {
                let version = versions[choice].clone();
                command_version_registry.add(version)?;
            }

            command_version_registry
                .save()
                .context("Failed to save command version registry")?;

            shim::make_shim(command, env::current_exe().unwrap().as_path())
                .unwrap_or_else(|err| panic!("failed to create shim for {command}: {err}"));
        }
    }

    Ok(())
}
