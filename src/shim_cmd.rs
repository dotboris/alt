use crate::environment::load_command_version_registry;
use crate::shim;
use console::style;
use std::env;

pub fn run() {
    shim::empty_shim_dir().expect("failed to empty shim dir");

    let command_version_registry =
        load_command_version_registry().expect("TODO: better error handling");

    for command in command_version_registry.command_names() {
        let res = shim::make_shim(&command, &env::current_exe().unwrap());
        match res {
            Ok(()) => println!(" {}️ {}", style("✓").green().bold(), command),
            Err(err) => println!(" {} {}: {}", style("✗").red().bold(), command, err),
        }
    }
}
