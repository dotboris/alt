#[macro_use]
extern crate clap;

use std::process;
mod versions;
mod exec_cmd;

fn values_of_or_empty(values: Option<clap::Values>) -> Vec<&str> {
    match values {
        Some(values) => values.collect(),
        None => vec![],
    }
}

fn main() {
    let matches = clap_app!(alt =>
        (version: crate_version!())
        (about: "Switch between different versions of commands")
        (@subcommand exec =>
            (about: "Run the specipied command")
            (@arg command: +required "The command to run")
            (@arg command_args: ... "Arguments to pass to the command")
        )
    ).get_matches();

    match matches.subcommand() {
        ("exec", Some(matches)) =>
            exec_cmd::run(
                matches.value_of("command").unwrap(),
                values_of_or_empty(matches.values_of("command_args"))
            ),
        ("", None) => {
            println!("Please specify a subcommand");
            // TODO: how do I display the full usage here?
            process::exit(1);
        },
        _ => unreachable!(),
    };
}
