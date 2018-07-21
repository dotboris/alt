use def_file;
use shim;
use std::env;
use console::style;

pub fn run() {
    let defs = def_file::load();
    for command in defs.keys() {
        let res = shim::make_shim(command, &env::current_exe().unwrap());
        match res {
            Ok(()) => println!(" {}️ {}", style("✓").green().bold(), command),
            Err(err) => println!(" {} {}: {}", style("✗").red().bold(), command, err),
        }
    }
}
