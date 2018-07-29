extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;
use std::env;
use std::path::*;
use assert_cmd::cargo;
mod test_env;

fn def_command(env: &Path, alt_bin: &Path, command: &str, version: &str, path: &Path) -> Command {
    let mut c = Command::new(alt_bin);
    test_env::set_env(&mut c, env);
    c.args(&["def", command, version, path.to_str().unwrap()]);
    c
}

fn use_command(env: &Path, alt_bin: &Path, command: &str, version: &str) -> Command {
    let mut c = Command::new(alt_bin);
    test_env::set_env(&mut c, env);
    c.args(&["use", command, version]);
    c
}

fn command(env: &Path, name: &str) -> Command {
    let mut c = Command::new(name);
    test_env::set_env(&mut c, env);
    c
}

#[test]
fn def_and_use() {
    let alt_bin = cargo::main_binary_path().unwrap();
    let env = test_env::create_env();

    def_command(
        &env, &alt_bin,
        "alfa", "1",
        &test_env::fixture_command_path("alfa", 1)
    )
        .assert()
        .success();

    env::set_current_dir(&env).unwrap();

    command(&env, "alfa")
        .assert()
        .failure()
        .stdout("command not found: alfa\n");

    use_command(&env, &alt_bin, "alfa", "1")
        .assert()
        .success();

    command(&env, "alfa")
        .assert()
        .success()
        .stdout("alfa1\n");

    test_env::delete_env(env);
}
