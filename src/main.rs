#[macro_use]
extern crate clap;

mod config;
mod use_file;
mod def_file;
mod exec_cmd;
mod cli;
mod shim;

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
