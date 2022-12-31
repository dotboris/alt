use crate::config;
use crate::definitions::Definitions;
use crate::scan;
use crate::scan::CommandVersion;
use crate::shim;
use dialoguer::MultiSelect;
use std::env;
use std::process;

fn prompt_versions(versions: &[CommandVersion]) -> Vec<usize> {
    let items: Vec<_> = versions
        .iter()
        .map(|version| {
            format!(
                "{} {} ({})",
                version.command,
                version.version,
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

pub fn run(command: &str) {
    let versions: Vec<_> = scan::path_suffix::scan(command)
        .into_iter()
        .chain(scan::homebrew::scan(command))
        .collect();

    if versions.is_empty() {
        println!("Sorry, could not find any versions of {}", command);
        process::exit(1);
    } else {
        let choices = prompt_versions(&versions);

        if choices.is_empty() {
            println!("Looks like you didn't choose anything.");
            println!("Did you forget to select versions with <space>?");
        } else {
            let definitions_file_path = config::definitions_file();
            let mut definitions =
                Definitions::load_or_default(&definitions_file_path).expect("TODO: error handling");

            for choice in choices {
                let version = &versions[choice];
                definitions.add_version(&version.command, &version.version, &version.path);
            }

            definitions
                .save(&definitions_file_path)
                .expect("TODO: error handling");

            shim::make_shim(command, env::current_exe().unwrap().as_path())
                .unwrap_or_else(|err| panic!("failed to create shim for {}: {}", command, err));
        }
    }
}
