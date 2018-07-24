extern crate toml;
extern crate dialoguer;

use config;
use std::env;
use std::fs;
use std::path::*;
use regex::Regex;
use std::process;
use def_file;
use shim;
use scan;
use scan::CommandVersion;

fn prompt_versions(versions: &Vec<CommandVersion>) -> Vec<usize> {
    let items: Vec<_> = versions.iter()
        .map(|version| format!("{} {} ({})",
            version.command, version.version, version.path.to_str().unwrap()
        ))
        .collect();

    let items_refs: Vec<_> = items.iter().map(String::as_ref).collect();

    println!("Here are the versions I found.");
    println!("  ↑/↓,j/k: move cursor");
    println!("  <space>: toggle keep");
    println!("  <enter>: confirm");
    println!();

    dialoguer::Checkboxes::new()
        .items(items_refs.as_slice())
        .clear(false)
        .interact()
        .unwrap()
}

pub fn run(command: &str) {
    let scans = vec![
        scan::path_suffix::scan(command)
    ];
    let is_empty = scans.iter()
        .all(|v| v.is_empty());
    let versions: Vec<_> = scans.into_iter().flat_map(|x| x).collect();

    if is_empty {
        println!("Sorry, could not find any versions of {}", command);
        process::exit(1);
    } else {
        let choices = prompt_versions(&versions);

        if choices.is_empty() {
            println!("Looks like you didn't choose anything.");
            println!("Did you forget to select versions with <space>?");
        } else {
            let mut defs = def_file::load();
            {
                let def = defs.entry(command.to_string())
                    .or_insert_with(|| def_file::CommandVersions::new());

                for choice in choices {
                    let version = &versions[choice];
                    def.insert(version.version.clone(), version.path.clone());
                }
            }
            def_file::save(&defs)
                .expect("failed to save defs file");

            shim::make_shim(command, env::current_exe().unwrap().as_path())
                .expect(&format!("failed to create shim for {}", command));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_only_should_not_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python"));
        assert_eq!(None, res)
    }

    #[test]
    fn command_with_simple_number_suffix_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python2"));
        assert_eq!(res, Some(CommandVersion {
            command: String::from("python"),
            version: String::from("2"),
            path: PathBuf::from("/usr/bin/python2"),
        }))
    }

    #[test]
    fn command_with_version_suffix_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python2.7"));
        assert_eq!(res, Some(CommandVersion {
            command: String::from("python"),
            version: String::from("2.7"),
            path: PathBuf::from("/usr/bin/python2.7"),
        }))
    }

    #[test]
    fn command_with_version_suffix_and_dash_should_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/ruby-2.5"));
        assert_eq!(res, Some(CommandVersion {
            command: String::from("ruby"),
            version: String::from("2.5"),
            path: PathBuf::from("/usr/bin/ruby-2.5"),
        }))
    }

    #[test]
    fn command_with_text_suffix_should_not_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/python-config"));
        assert_eq!(res, None);
    }

    #[test]
    fn command_trailing_period_in_suffix_should_not_parse() {
        let res = parse_command_version(PathBuf::from("/usr/bin/something-2.1."));
        assert_eq!(res, None);
    }
}
