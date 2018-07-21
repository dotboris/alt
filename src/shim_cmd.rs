use def_file;
use shim;
use std::env;

pub fn run() {
    let defs = def_file::load();
    for command in defs.keys() {
        let res = shim::make_shim(command, &env::current_exe().unwrap());
        match res {
            Ok(()) => println!(" ✓️ {}", command),
            Err(err) => println!(" ✗ {}: {}", command, err),
        }
    }
}
