use crate::command_version::CommandVersionRegistry;
use crate::config;
use crate::shim;
use console::style;
use std::env;

pub fn run() {
    shim::empty_shim_dir().expect("failed to empty shim dir");

    let command_version_registry =
        CommandVersionRegistry::load_or_default(&config::definitions_file())
            .expect("TODO: better error handling");

    for command in command_version_registry.command_names() {
        let res = shim::make_shim(&command, &env::current_exe().unwrap());
        match res {
            Ok(()) => println!(" {}️ {}", style("✓").green().bold(), command),
            Err(err) => println!(" {} {}: {}", style("✗").red().bold(), command, err),
        }
    }
}
