use clap::App;
use clap::AppSettings;

pub fn make_app() -> App<'static, 'static> {
    clap_app!(alt =>
        (version: crate_version!())
        (about: "Switch between different versions of commands")
        (@subcommand exec =>
            (about: "Run the given command")
            (after_help:
"ARGS NOTE:
    Note that `alt exec` handles some flags on its own (`--help` for example).
    If you want to pass such arguments to the executed command instead of
    `alt exec`, you will need to use `--` to tell alt exec to stop parsing
    arguments.

    Example:

    alt exec node --help        # --help passed to alt (shows this message)
    alt exec node -- --help     # --help passed to node (shows node's help)"
            )
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
            (@arg command: +required "Command to switch the version of")
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
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::VersionlessSubcommands
        ])
}
