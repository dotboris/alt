use def_file;
use command;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;

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
        .or_else(|| command::find_system_bin(command));

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
