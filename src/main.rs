#[macro_use]
extern crate clap;

mod config;
mod use_file;
mod def_file;
mod exec_cmd;
mod cli;

fn main() {
    cli::run();
}
