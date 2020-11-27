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
mod doctor_cmd;
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
        let matches = cli::make_app().get_matches();

        match matches.subcommand() {
            ("exec", Some(matches)) => {
                let args = matches.values_of("command_args")
                    .unwrap_or_default()
                    .map(|i| i.to_owned())
                    .collect::<Vec<String>>();

                exec_cmd::run(
                    matches.value_of("command").unwrap(),
                    &args
                )
            },
            ("which", Some(matches)) =>
                which_cmd::run(matches.value_of("command").unwrap()),
            ("shim", Some(_)) => shim_cmd::run(),
            ("scan", Some(matches)) =>
                scan_cmd::run(matches.value_of("command").unwrap()),
            ("use", Some(matches)) =>
                use_cmd::run(
                    matches.value_of("command").unwrap(),
                    matches.value_of("version")
                ),
            ("show", Some(_)) => show_cmd::run(),
            ("doctor", Some(matches)) => {
                let fix = match matches.value_of("fix_mode") {
                    Some("auto") => doctor_cmd::FixMode::Auto,
                    Some("never") => doctor_cmd::FixMode::Never,
                    Some("prompt") => doctor_cmd::FixMode::Prompt,
                    _ => unreachable!()
                };
                doctor_cmd::run(fix)
            },
            ("def", Some(matches)) =>
                def_cmd::run(
                    matches.value_of("command").unwrap(),
                    matches.value_of("version").unwrap(),
                    matches.value_of("bin").unwrap()
                ),
            _ => unreachable!(),
        };
    }
}
