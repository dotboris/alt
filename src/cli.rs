use exec_cmd;
use shim_cmd;
use which_cmd;
use scan_cmd;
use use_cmd;
use show_cmd;
use def_cmd;
use clap::AppSettings;

pub fn run() {
    let matches = clap_app!(alt =>
        (version: crate_version!())
        (about: "Switch between different versions of commands")
        (@subcommand exec =>
            (about: "Run the given command")
            (@arg command: +required "The command to run")
            (@arg command_args: ... "Arguments to pass to the command")
        )
        (@subcommand shim =>
            (about: "Generate shims for all managed commands")
        )
        (@subcommand which =>
            (about: "Print the resolved path of a command")
            (@arg command: +required "Command to look up")
        )
        (@subcommand scan =>
            (about: "Scan for different versions of the given command")
            (@arg command: +required "Command to scan for")
        )
        (@subcommand use =>
            (about: "Swtich the version of a command")
            (after_help:
"EXAMPLES:
    alt use node 8        Use version 8 of node
    alt use node          Prompt for a version of node to use
    alt use node system   Use the system version of node"
            )
            (@arg command: +required "Command who's version to switch")
            (@arg version: "Version to use (optional)")
        )
        (@subcommand show =>
            (about: "Print commands and their versions")
        )
        (@subcommand def =>
            (about: "Define a new version")
            (@arg command: +required "Command to define the version for")
            (@arg version: +required "The name of the version")
            (@arg bin: +required "Path to the executable for the version")
        )
    )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

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
        ("def", Some(matches)) =>
            def_cmd::run(
                matches.value_of("command").unwrap(),
                matches.value_of("version").unwrap(),
                matches.value_of("bin").unwrap()
            ),
        _ => unreachable!(),
    };
}
