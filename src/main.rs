mod checks;
mod cli;
mod command;
mod config;
mod def_cmd;
mod def_file;
mod definitions;
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
                    .get_many::<String>("command_args")
                    .unwrap_or_default()
                    .map(String::to_owned)
                    .collect::<Vec<String>>();

                exec_cmd::run(matches.get_one::<String>("command").unwrap(), &args)
            }
            Some(("which", matches)) => {
                which_cmd::run(matches.get_one::<String>("command").unwrap())
            }
            Some(("shim", _)) => shim_cmd::run(),
            Some(("scan", matches)) => scan_cmd::run(matches.get_one::<String>("command").unwrap()),
            Some(("use", matches)) => use_cmd::run(
                matches.get_one::<String>("command").unwrap(),
                matches.get_one::<String>("version").map(String::as_ref),
            ),
            Some(("show", _)) => show_cmd::run(),
            Some(("doctor", matches)) => {
                let fix_mode = match matches.get_one::<String>("fix_mode").map(String::as_ref) {
                    Some("auto") => doctor_cmd::FixMode::Auto,
                    Some("never") => doctor_cmd::FixMode::Never,
                    Some("prompt") => doctor_cmd::FixMode::Prompt,
                    _ => unreachable!(),
                };
                doctor_cmd::run(fix_mode)
            }
            Some(("def", matches)) => def_cmd::run(
                matches.get_one::<String>("command").unwrap(),
                matches.get_one::<String>("version").unwrap(),
                matches.get_one::<String>("bin").unwrap(),
            ),
            _ => unreachable!(),
        };
    }
}
