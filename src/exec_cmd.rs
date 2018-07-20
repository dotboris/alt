use use_file;
use def_file;
use std::env;

pub fn run(command: &str, command_args: Vec<&str>) {
    let command = String::from(command);
    println!("TODO: exec {} {:?}", command, command_args);
    let command_def = def_file::load_for(&command);
    println!("{:?}", command_def);
    let file = use_file::find(env::current_dir().unwrap());
    println!("{:?}", file);
    let bin = match file {
        Some(path) =>
            use_file::load(path)
                .get(&command)
                .map(|c| c.clone())
                .unwrap_or(command),
        None => command,
    };

    println!("{:?}", bin)
}
