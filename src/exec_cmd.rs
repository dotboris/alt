use versions;
use std::env;

pub fn run(command: &str, command_args: Vec<&str>) {
    println!("TODO: exec {} {:?}", command, command_args);
    let command_def = versions::load_def_for(command);
    println!("{:?}", command_def);
    let file = versions::find_use_file(env::current_dir().unwrap());
    println!("{:?}", file);
    let bin = match file {
        Some(file) => "foobar",
        None => command,
    };
}
