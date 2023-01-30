use crate::command::find_selected_binary;
use crate::environment::load_command_version_registry;
use std::process;

pub fn run(command: &str) -> anyhow::Result<()> {
    let command_version_registry = load_command_version_registry()?;

    match find_selected_binary(&command_version_registry, command) {
        Some(bin) => println!("{}", bin.to_str().unwrap()),
        None => {
            println!("command not found: {command}");
            process::exit(1)
        }
    };

    Ok(())
}
