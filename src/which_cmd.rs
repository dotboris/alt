use def_file;
use command;
use std::process;
use std::env;

pub fn run(command: &str) {
    let defs = def_file::load();
    let command_version = command::find_selected_version(&command);

    let bin = command_version
        .and_then(|version| def_file::find_bin(&defs, command, &version))
        .map(|bin| bin.to_owned())
        .or_else(|| {
            let path = env::var("PATH").expect("env var PATH is not defined");
            let current_exe = env::current_exe().unwrap();
            command::find_system_bin(command, &path, &current_exe)
        });

    match bin {
        Some(bin) => println!("{}", bin.to_str().unwrap()),
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        },
    };
}
