use crate::def_file;
use crate::command;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;
use std::env;

pub fn run(command: &str, command_args: &[String]) {
    let defs = def_file::load();
    let command_version = command::find_selected_version(&command);

    let bin = command_version
        .clone()
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

            // Since we're calling exec, either our process will be replaced
            // (and this code will never be called) or something's wrong and
            // we get this error
            eprintln!("🔥 alt failed to run {}!", command);
            eprintln!("error: {:?}", err);
            eprintln!("command: {}", command);
            eprintln!("command version: {}",
                command_version
                    .as_deref()
                    .unwrap_or("(using system version)")
            );
            eprintln!("args: {:?}", command_args);
            eprintln!("bin: {}", bin.display());
            eprintln!("current dir: {:?}", env::current_dir());
            panic!();
        },
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        },
    }
}
