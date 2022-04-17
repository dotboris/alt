use clap::{ App, AppSettings, SubCommand, Arg, crate_version };

pub fn make_app() -> App<'static> {
    return App::new("alt")
        .version(crate_version!())
        .about("Switch between different versions of commands")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])

        .subcommand(SubCommand::with_name("exec")
            .about("Run the given command")
            .after_help(
"ARGS NOTE:
    Note that `alt exec` handles some flags on its own (`--help` for example).
    If you want to pass such arguments to the executed command instead of
    `alt exec`, you will need to use `--` to tell alt exec to stop parsing
    arguments.

    Example:

    alt exec node --help        # --help passed to alt (shows this message)
    alt exec node -- --help     # --help passed to node (shows node's help)"
            )
            .arg(Arg::with_name("command")
                .help("The command to run")
                .required(true)
            )
            .arg(Arg::with_name("command_args")
                .help("Arguments to pass to the command")
                .multiple(true),
            )
        )

        .subcommand(SubCommand::with_name("shim")
            .about("Generate shims for all managed commands")
        )

        .subcommand(SubCommand::with_name("which")
            .about("Print the resolved path of a command")
            .arg(Arg::with_name("command")
                .required(true)
                .help("Command to look up")
            )
        )

        .subcommand(SubCommand::with_name("scan")
            .about("Scan for different versions of the given command")
            .arg(Arg::with_name("command")
                .required(true)
                .help("Command to scan for")
            )
        )

        .subcommand(SubCommand::with_name("use")
            .about("Switch the version of a command")
            .after_help(
"EXAMPLES:
    alt use node 8        Use version 8 of node
    alt use node          Prompt for a version of node to use
    alt use node system   Use the system version of node"
            )
            .arg(Arg::with_name("command")
                .required(true)
                .help("Command to switch the version of")
            )
            .arg(Arg::with_name("version")
                .help("Version to use (optional)")
            )
        )

        .subcommand(SubCommand::with_name("show")
            .about("Print commands and their versions")
        )

        .subcommand(SubCommand::with_name("def")
            .about("Define a new version")
            .arg(Arg::with_name("command")
                .required(true)
                .help("Command to define the version for")
            )
            .arg(Arg::with_name("version")
                .required(true)
                .help("The name of the version")
            )
            .arg(Arg::with_name("bin")
                .required(true)
                .help("Path to the executable for the version")
            )
        )

        .subcommand(SubCommand::with_name("doctor")
            .about("Checks if alt is setup correctly. Helps debug problems.")
            .arg(Arg::with_name("fix_mode")
                .short('f')
                .long("fix-mode")
                .takes_value(true)
                .possible_values(&["auto", "never", "prompt"])
                .default_value("prompt")
                .help("Control how automatic fixes are applied.")
            )
        )
        ;
}
