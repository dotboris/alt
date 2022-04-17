use clap::{crate_version, Arg, Command};

pub fn make_app() -> Command<'static> {
    return Command::new("alt")
        .version(crate_version!())
        .about("Switch between different versions of commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("exec")
                .about("Run the given command")
                .after_help(
                    "ARGS NOTE:
    Note that `alt exec` handles some flags on its own (`--help` for example).
    If you want to pass such arguments to the executed command instead of
    `alt exec`, you will need to use `--` to tell alt exec to stop parsing
    arguments.

    Example:

    alt exec node --help        # --help passed to alt (shows this message)
    alt exec node -- --help     # --help passed to node (shows node's help)",
                )
                .arg(
                    Arg::new("command")
                        .help("The command to run")
                        .required(true),
                )
                .arg(
                    Arg::new("command_args")
                        .help("Arguments to pass to the command")
                        .multiple_values(true),
                ),
        )
        .subcommand(Command::new("shim").about("Generate shims for all managed commands"))
        .subcommand(
            Command::new("which")
                .about("Print the resolved path of a command")
                .arg(
                    Arg::new("command")
                        .required(true)
                        .help("Command to look up"),
                ),
        )
        .subcommand(
            Command::new("scan")
                .about("Scan for different versions of the given command")
                .arg(
                    Arg::new("command")
                        .required(true)
                        .help("Command to scan for"),
                ),
        )
        .subcommand(
            Command::new("use")
                .about("Switch the version of a command")
                .after_help(
                    "EXAMPLES:
    alt use node 8        Use version 8 of node
    alt use node          Prompt for a version of node to use
    alt use node system   Use the system version of node",
                )
                .arg(
                    Arg::new("command")
                        .required(true)
                        .help("Command to switch the version of"),
                )
                .arg(Arg::new("version").help("Version to use (optional)")),
        )
        .subcommand(Command::new("show").about("Print commands and their versions"))
        .subcommand(
            Command::new("def")
                .about("Define a new version")
                .arg(
                    Arg::new("command")
                        .required(true)
                        .help("Command to define the version for"),
                )
                .arg(
                    Arg::new("version")
                        .required(true)
                        .help("The name of the version"),
                )
                .arg(
                    Arg::new("bin")
                        .required(true)
                        .help("Path to the executable for the version"),
                ),
        )
        .subcommand(
            Command::new("doctor")
                .about("Checks if alt is setup correctly. Helps debug problems.")
                .arg(
                    Arg::new("fix_mode")
                        .short('f')
                        .long("fix-mode")
                        .takes_value(true)
                        .possible_values(&["auto", "never", "prompt"])
                        .default_value("prompt")
                        .help("Control how automatic fixes are applied."),
                ),
        );
}
