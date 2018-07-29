extern crate assert_cmd;

use assert_cmd::prelude::*;
mod test_env;
use test_env::TestEnv;
use std::fs;

fn def_all(env: &TestEnv) {
    for command in &["alfa", "bravo", "charlie"] {
        for version in &[1, 2, 3] {
            env.def(
                command, &version.to_string(),
                &env.fixture_path(command, version.clone())
            )
                .assert()
                .success();
        }
    }
}

#[test]
fn def_and_use() {
    let env = TestEnv::new();
    def_all(&env);

    env._use("alfa", "1")
        .assert()
        .success();

    env.command("alfa")
        .assert()
        .success()
        .stdout("alfa1\n");
}

#[test]
fn system_with_no_use() {
    let env = TestEnv::new();
    def_all(&env);

    env.command("bravo")
        .assert()
        .success()
        .stdout("bravo system\n");
}

#[test]
fn reset_with_use_system() {
    let env = TestEnv::new();
    def_all(&env);

    env._use("charlie", "3")
        .assert()
        .success();

    env.command("charlie")
        .assert()
        .success()
        .stdout("charlie3\n");

    env._use("charlie", "system")
        .assert()
        .success();

    env.command("charlie")
        .assert()
        .success()
        .stdout("charlie system\n");
}

#[test]
fn use_with_subdir() {
    let env = TestEnv::new();
    def_all(&env);

    env._use("alfa", "3")
        .assert()
        .success();

    let subdir = env.root.join("subdir");
    fs::create_dir(&subdir).unwrap();
    env.command("alfa")
        .current_dir(&subdir)
        .assert()
        .success()
        .stdout("alfa3\n");
}

#[test]
fn use_with_subdir_overwrite() {
    let env = TestEnv::new();
    def_all(&env);

    env._use("bravo", "1")
        .assert()
        .success();

    env.command("bravo")
        .assert()
        .success()
        .stdout("bravo1\n");

    let subdir = env.root.join("subdir");
    fs::create_dir(&subdir).unwrap();

    env._use("bravo", "2")
        .assert()
        .success();

    env.command("bravo")
        .current_dir(&subdir)
        .assert()
        .success()
        .stdout("bravo2\n");
}
