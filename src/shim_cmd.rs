use crate::def_file;
use crate::shim;
use console::style;
use std::env;

pub fn run() {
    shim::empty_shim_dir().expect("failed to empty shim dir");

    let defs = def_file::load();
    for command in defs.keys() {
        let res = shim::make_shim(command, &env::current_exe().unwrap());
        match res {
            Ok(()) => println!(" {}️ {}", style("✓").green().bold(), command),
            Err(err) => println!(" {} {}: {}", style("✗").red().bold(), command, err),
        }
    }
}
