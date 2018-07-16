use versions;
use std::env;

pub fn exec(command: &str, command_args: Vec<&str>) {
    println!("TODO: exec {} {:?}", command, command_args);
    let file = versions::find_file(env::current_dir().unwrap());
    println!("{:?}", file);
    let bin = match file {
        Some(file) => "foobar",
        None => command,
    };
}
