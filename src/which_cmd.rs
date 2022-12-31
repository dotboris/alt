use crate::command::find_selected_binary;
use crate::config;
use crate::definitions::Definitions;
use std::process;

pub fn run(command: &str) {
    let definitions =
        Definitions::load_or_default(&config::definitions_file()).expect("TODO: better errors");

    match find_selected_binary(&definitions, command) {
        Some(bin) => println!("{}", bin.to_str().unwrap()),
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        }
    };
}
