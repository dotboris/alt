#[macro_use]
extern crate clap;

use std::process;

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

    let exit_code: Option<i32> = match matches.subcommand() {
        ("exec", Some(matches)) => {
            let command = matches.value_of("command").unwrap();
            let command_args = values_of_or_empty(matches.values_of("command_args"));

            println!("TODO: exec {} {:#?}", command, command_args);
            None
        },
        ("", None) => {
            println!("Please specify a subcommand");
            // TODO: how do I display the full usage here?
            Some(1)
        },
        _ => unreachable!(),
    };

    process::exit(exit_code.unwrap_or(0));
}
