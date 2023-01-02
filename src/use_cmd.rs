use crate::command_version::CommandVersion;
use crate::command_version::CommandVersionRegistry;
use crate::config;
use crate::use_file;
use dialoguer::Select;
use std::env;
use std::process;

enum SelectedVersion {
    System,
    ThisOne(CommandVersion),
}

pub fn run(command: &str, arg_version: Option<&str>) {
    let registry =
        CommandVersionRegistry::load_or_default(&config::definitions_file()).expect("TODO: errors");

    let command_versions = registry
        .iter()
        .filter(|v| v.command_name == command)
        .collect::<Vec<_>>();

    if command_versions.is_empty() {
        println!("Unknown command {}", command);
        println!("Did you forget to define it? (see alt help scan)");
        process::exit(1);
    }

    let selected_version = match arg_version {
        Some("system") => SelectedVersion::System,
        Some(version) => SelectedVersion::ThisOne(registry.get(command, version).unwrap()),
        None => prompt_version(&command_versions),
    };

    match selected_version {
        SelectedVersion::System => {
            let cwd = env::current_dir().unwrap();
            let use_file = use_file::find_or_dir(&cwd);
            let mut use_def = use_file::load(&use_file).unwrap_or_default();
            use_def.remove(command);
            use_file::save(&use_def, &use_file).unwrap_or_else(|err| {
                panic!(
                    "Failed to write use file to {}: {}",
                    use_file.to_str().unwrap(),
                    err
                )
            });

            println!(
                "Will now use system version of {} when in {}",
                command,
                use_file.parent().unwrap().to_str().unwrap()
            );
        }
        SelectedVersion::ThisOne(CommandVersion {
            command_name,
            version_name,
            path,
        }) => {
            let cwd = env::current_dir().unwrap();
            let use_file = use_file::find_or_dir(&cwd);
            let mut use_def = use_file::load(&use_file).unwrap_or_default();
            use_def.insert(command_name.clone(), version_name.clone());
            use_file::save(&use_def, &use_file).unwrap_or_else(|err| {
                panic!(
                    "Failed to write use file to {}: {}",
                    use_file.to_str().unwrap(),
                    err
                )
            });

            println!(
                "Will now use {} {} ({}) when in {}",
                command_name,
                version_name,
                path.display(),
                use_file.parent().unwrap().to_str().unwrap()
            );
        }
    }
}

fn prompt_version(versions: &[CommandVersion]) -> SelectedVersion {
    let mut versions = versions.to_owned();
    versions.sort();

    println!("Please select a version to use");
    println!("  ↑/↓,j/k: move cursor");
    println!("  <enter>: select");
    println!();

    let mut version_strings: Vec<_> = versions
        .iter()
        .map(|c| format!("{} ({})", c.version_name, c.path.display()))
        .collect();
    version_strings.insert(0, "system version".to_string());

    let choice = Select::new()
        .items(&version_strings)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => SelectedVersion::System,
        i => SelectedVersion::ThisOne(versions[i - 1].clone()),
    }
}
