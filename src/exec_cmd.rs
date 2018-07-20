use use_file;
use def_file;
use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn run<'a>(command: &str, command_args: Vec<&str>) {
    let system_command = String::from(command);

    let command_def = def_file::load_for(&command);

    let file = use_file::find(env::current_dir().unwrap())
        .map(|path| use_file::load(&path));
    let command_version = file.as_ref()
        .and_then(|file| file.get(command));

    let bin = command_version
        .and_then(|v| command_def.get(v))
        .unwrap_or(&system_command);

    let err = Command::new(&bin)
        .args(command_args)
        .exec();

    panic!("Failed to exec(): {:#?}", err)
}
