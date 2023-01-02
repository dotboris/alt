use crate::command_version::CommandVersionRegistry;
use crate::config;
use crate::use_file;
use console::style;
use std::env;
use std::process;

pub fn run() {
    let registry = CommandVersionRegistry::load_or_default(&config::definitions_file())
        .expect("TODO: better errors");

    if registry.is_empty() {
        println!("No commands are defined.");
        println!("Try alt scan");
        process::exit(1);
    }

    let use_file = use_file::find(&env::current_dir().unwrap());
    let used_versions = use_file
        .as_ref()
        .and_then(|path| use_file::load(path))
        .unwrap_or_default();

    if let Some(use_file_path) = use_file {
        println!("Versions from: {}", use_file_path.to_str().unwrap());
    }

    let mut command_versions = registry.iter().collect::<Vec<_>>();
    command_versions.sort_by(|a, b| {
        (&a.command_name, &a.version_name).cmp(&(&b.command_name, &b.version_name))
    });

    let mut current_command: Option<String> = None;
    for command_version in command_versions {
        let currently_used_version = used_versions.get(&command_version.command_name);

        if current_command.as_ref() != Some(&command_version.command_name) {
            current_command = Some(command_version.command_name.clone());

            let command_display = style(&command_version.command_name).bold();
            if currently_used_version.is_some() {
                println!("{}", command_display);
            } else {
                println!("{} {}", command_display, style("(using system)").yellow());
            }
        }

        if currently_used_version == Some(&command_version.version_name) {
            println!(
                " {} {} ({})",
                style("*").green().bold(),
                &command_version.version_name,
                command_version.path.display()
            );
        } else {
            println!(
                "   {} ({})",
                &command_version.version_name,
                command_version.path.display()
            );
        }
    }
}
