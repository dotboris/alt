extern crate clap;
extern crate console;
#[macro_use]
extern crate lazy_static;
extern crate glob;
extern crate regex;

mod checks;
mod cli;
mod command;
mod config;
mod def_cmd;
mod def_file;
mod doctor_cmd;
mod exec_cmd;
mod scan;
mod scan_cmd;
mod shim;
mod shim_cmd;
mod show_cmd;
mod use_cmd;
mod use_file;
mod which_cmd;

use std::env;

fn main() {
    checks::check_shim_in_path();

    let arg0 = env::args().next().unwrap();

    if shim::is_shim(&arg0) {
        let args = env::args().skip(1).collect::<Vec<String>>();

        exec_cmd::run(shim::get_command(&arg0), &args);
    } else {
        let matches = cli::make_app().get_matches();

        match matches.subcommand() {
            Some(("exec", matches)) => {
                let args = matches
                    .values_of("command_args")
                    .unwrap_or_default()
                    .map(|i| i.to_owned())
                    .collect::<Vec<String>>();

                exec_cmd::run(matches.value_of("command").unwrap(), &args)
            }
            Some(("which", matches)) => which_cmd::run(matches.value_of("command").unwrap()),
            Some(("shim", _)) => shim_cmd::run(),
            Some(("scan", matches)) => scan_cmd::run(matches.value_of("command").unwrap()),
            Some(("use", matches)) => use_cmd::run(
                matches.value_of("command").unwrap(),
                matches.value_of("version"),
            ),
            Some(("show", _)) => show_cmd::run(),
            Some(("doctor", matches)) => {
                let fix_mode = match matches.value_of("fix_mode") {
                    Some("auto") => doctor_cmd::FixMode::Auto,
                    Some("never") => doctor_cmd::FixMode::Never,
                    Some("prompt") => doctor_cmd::FixMode::Prompt,
                    _ => unreachable!(),
                };
                doctor_cmd::run(fix_mode)
            }
            Some(("def", matches)) => def_cmd::run(
                matches.value_of("command").unwrap(),
                matches.value_of("version").unwrap(),
                matches.value_of("bin").unwrap(),
            ),
            _ => unreachable!(),
        };
    }
}
