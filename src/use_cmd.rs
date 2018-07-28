extern crate dialoguer;

use def_file;
use use_file;
use std::process;
use std::env;
use self::dialoguer::Select;

pub fn run(command: &str, arg_version: Option<&str>) {
    let defs = def_file::load();

    let command_versions = defs.get(command)
        .unwrap_or_else(|| {
            println!("Unknown command {}", command);
            println!("Did you forget to define it? (see alt help scan)");
            process::exit(1);
        });

    let version = arg_version
        .unwrap_or_else(|| prompt_version(command_versions));

    if version == "system" {
        let cwd = env::current_dir().unwrap();
        let use_file = use_file::find_or_dir(&cwd);
        let mut use_def = use_file::load(&use_file).unwrap_or_default();
        use_def.remove(command);
        use_file::save(&use_def, &use_file)
            .expect(&format!("Failed to write use file to {}", use_file.to_str().unwrap()));

        println!(
            "Will now use system version of {} when in {}",
            command,
            use_file.parent().unwrap().to_str().unwrap()
        );
    } else {
        let bin = command_versions.get(version)
            .unwrap_or_else(|| {
                println!("Unknown version {} for command {}", version, command);
                println!("Did you forget to define it? (see alt help scan)");
                process::exit(2);
            });

        let cwd = env::current_dir().unwrap();
        let use_file = use_file::find_or_dir(&cwd);
        let mut use_def = use_file::load(&use_file).unwrap_or_default();
        use_def.insert(String::from(command), String::from(version));
        use_file::save(&use_def, &use_file)
            .expect(&format!("Failed to write use file to {}", use_file.to_str().unwrap()));

        println!(
            "Will now use {} {} ({}) when in {}",
            command,
            version,
            bin.to_str().unwrap(),
            use_file.parent().unwrap().to_str().unwrap()
        );
    }
}

fn prompt_version(versions: &def_file::CommandVersions) -> &str {
    let versions_vec: Vec<_> = versions.iter().collect();
    let version_strings: Vec<_> = versions_vec.iter()
        .map(|(version, bin)| format!("{} ({})",
            version, bin.to_str().unwrap()
        ))
        .collect();

    println!("Please select a version to use");
    println!("  ↑/↓,j/k: move cursor");
    println!("  <enter>: select");
    println!();

    let mut items_refs: Vec<&str> = version_strings.iter().map(String::as_ref).collect();
    items_refs.insert(0, "system version");
    let choice = Select::new()
        .items(items_refs.as_slice())
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => "system",
        i => versions_vec[i - 1].0,
    }
}
