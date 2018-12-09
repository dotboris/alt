#[macro_use]
extern crate clap;
extern crate console;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate glob;

mod config;
mod use_file;
mod def_file;
mod exec_cmd;
mod shim_cmd;
mod which_cmd;
mod scan_cmd;
mod use_cmd;
mod show_cmd;
mod def_cmd;
mod cli;
mod shim;
mod command;
mod scan;
mod checks;

use std::env;

fn main() {
    checks::check_shim_in_path();

    let arg0 = env::args().next().unwrap();

    if shim::is_shim(&arg0) {
        let args = env::args()
            .skip(1)
            .collect::<Vec<String>>();

        exec_cmd::run(
            shim::get_command(&arg0),
            &args
        );
    } else {
        cli::run();
    }
}
