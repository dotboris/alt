extern crate assert_cmd;
extern crate predicates;

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::env;
use assert_cmd::cargo;
mod test_env;

#[test]
fn def_and_use() {
    let alt_bin = cargo::main_binary_path().unwrap();
    let env = test_env::create_env();

    for name in &["alfa", "bravo", "charlie"] {
        for version in &[1, 2, 3] {
            let mut command = Command::new(&alt_bin);
            test_env::set_env(&mut command, &env);
            command
                .args(&[
                    "def",
                    name,
                    &version.to_string(),
                    test_env::fixture_command_path(name, version)
                        .to_str()
                        .unwrap()
                ])
                .assert()
                .success();
        }
    }

    for (name, version) in &[("alfa", "1"), ("bravo", "2"), ("charlie", "3")] {
        env::set_current_dir(&env).unwrap();

        let mut command = Command::new(name);
        test_env::set_env(&mut command, &env);
        command
            .assert()
            .failure()
            .stdout(predicate::str::contains("command not found:").from_utf8());

        let mut alt_command = Command::new(&alt_bin);
        test_env::set_env(&mut alt_command, &env);
        alt_command
            .args(&["use", name, version])
            .assert()
            .success();

        let mut command = Command::new(name);
        test_env::set_env(&mut command, &env);
        command
            .assert()
            .success()
            .stdout(predicate::str::contains(format!("{}{}", name, version)).from_utf8());
    }

    test_env::delete_env(env);
}
