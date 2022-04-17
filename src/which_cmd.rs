use crate::command;
use crate::def_file;
use std::process;

pub fn run(command: &str) {
    let defs = def_file::load();
    let command_version = command::find_selected_version(command);

    let bin = command_version
        .and_then(|version| def_file::find_bin(&defs, command, &version))
        .map(|bin| bin.to_owned())
        .or_else(|| command::find_system_bin(command));

    match bin {
        Some(bin) => println!("{}", bin.to_str().unwrap()),
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        }
    };
}
