use use_file;
use def_file;
use std::env;

pub fn run(command: &str, command_args: Vec<&str>) {
    println!("TODO: exec {} {:?}", command, command_args);
    let command_def = def_file::load_for(command);
    println!("{:?}", command_def);
    let file = use_file::find(env::current_dir().unwrap());
    println!("{:?}", file);
    let bin = match file {
        Some(file) => "foobar",
        None => command,
    };
}
