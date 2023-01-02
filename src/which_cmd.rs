use crate::command::find_selected_binary;
use crate::config;
use crate::command_version::CommandVersionRegistry;
use std::process;

pub fn run(command: &str) {
    let command_version_registry =
        CommandVersionRegistry::load_or_default(&config::definitions_file())
            .expect("TODO: better errors");

    match find_selected_binary(&command_version_registry, command) {
        Some(bin) => println!("{}", bin.to_str().unwrap()),
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        }
    };
}
