use exec_cmd;
use shim_cmd;
use std::process;

pub fn run() {
    let matches = clap_app!(alt =>
        (version: crate_version!())
        (about: "Switch between different versions of commands")
        (@subcommand exec =>
            (about: "Run the specipied command")
            (@arg command: +required "The command to run")
            (@arg command_args: ... "Arguments to pass to the command")
        )
        (@subcommand shim =>
            (about: "Generate shims for all managed commands")
        )
    ).get_matches();

    match matches.subcommand() {
        ("exec", Some(matches)) => {
            let args = matches.values_of("command_args")
                .unwrap_or_default()
                .map(|i| i.to_owned())
                .collect();

            exec_cmd::run(
                matches.value_of("command").unwrap(),
                &args
            )
        },
        ("shim", Some(_)) => shim_cmd::run(),
        ("", None) => {
            println!("Please specify a subcommand");
            // TODO: how do I display the full usage here?
            process::exit(1);
        },
        _ => unreachable!(),
    };
}