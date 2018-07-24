use def_file;
use use_file;
use std::process;
use std::env;
use console::style;

pub fn run() {
    let defs = def_file::load();

    if defs.is_empty() {
        println!("No commands are defined.");
        println!("Try alt scan");
        process::exit(1);
    }

    let use_file = use_file::find(&env::current_dir().unwrap());
    let used_versions = use_file.as_ref()
        .and_then(|path| use_file::load(&path));

    if use_file.is_some() {
        println!("Versions from: {}", use_file.unwrap().to_str().unwrap());
    }

    let mut sorted_defs: Vec<_> = defs.iter().collect();
    sorted_defs.sort_by_key(|t| t.0);

    for (command, versions) in sorted_defs {
        let current_version = used_versions.as_ref()
            .and_then(|vs| vs.get(command));

        let command_display = style(command).bold();

        if current_version.is_some() {
            println!("{}", command_display);
        } else {
            println!("{} {}", command_display, style("(using system)").yellow());
        }

        let mut sorted_versions: Vec<_> = versions.iter().collect();
        sorted_versions.sort();

        for (version, bin) in sorted_versions {
            let bin_str = bin.to_str().unwrap();
            if current_version.is_some() && current_version.unwrap() == version {
                println!(" {} {} ({})", style("*").green().bold(), &version, bin_str);
            } else {
                println!("   {} ({})", &version, bin_str);
            }
        }
    }
}
