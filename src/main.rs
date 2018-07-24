#[macro_use]
extern crate clap;
extern crate console;
#[macro_use]
extern crate lazy_static;
extern crate regex;

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

use std::env;

fn main() {
    let arg0 = env::args().next().unwrap();

    if shim::is_shim(&arg0) {
        let args = env::args()
            .skip(1)
            .collect();

        exec_cmd::run(
            shim::get_command(&arg0),
            &args
        );
    } else {
        cli::run();
    }
}
