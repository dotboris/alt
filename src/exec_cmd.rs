use def_file;
use command;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn run(command: &str, command_args: &Vec<String>) {
    let system_command = String::from(command);

    let defs = def_file::load();
    let command_version = command::find_selected_version(&command);

    let bin = match command_version {
        Some(version) => {
            def_file::find_bin(&defs, command, &version)
                .expect(&format!("version {} of command {} is not defined",
                    version,
                    command
                ))
        },
        None => &system_command
    };

    let err = Command::new(&bin)
        .args(command_args)
        .exec();

    // Since we're callling exec, either our process will be replaced
    // (and this code will never be called) or something's wrong and
    // we get this error
    eprintln!("Failed to exec()!");
    eprintln!("{:#?}", err);
    panic!();
}
