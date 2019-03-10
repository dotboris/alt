use def_file;
use command;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;
use std::env;

pub fn run(command: &str, command_args: &[String]) {
    let defs = def_file::load();
    let command_version = command::find_selected_version(&command);

    let bin = command_version
        .map(|version| {
            def_file::find_bin(&defs, command, &version)
                .unwrap_or_else(|| panic!(
                    "version {} of command {} is not defined",
                    version,
                    command
                ))
                .to_owned()
        })
        .or_else(|| {
            let path = env::var("PATH").expect("env var PATH is not defined");
            let current_exe = env::current_exe().unwrap();
            command::find_system_bin(command, &path, &current_exe)
        });

    match bin {
        Some(bin) => {
            let err = Command::new(&bin)
                .args(command_args)
                .exec();

            // Since we're callling exec, either our process will be replaced
            // (and this code will never be called) or something's wrong and
            // we get this error
            eprintln!("Failed to exec()!");
            eprintln!("{:#?}", err);
            panic!();
        },
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        },
    }
}
