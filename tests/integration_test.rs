extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::env;
mod test_env;
use test_env::TestEnv;

fn def_all(env: &TestEnv) {
    for command in &["alfa", "bravo", "charlie"] {
        for version in &[1, 2, 3] {
            env.def(
                command, &version.to_string(),
                &test_env::fixture_command_path(command, version.clone())
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

    env::set_current_dir(&env.root).unwrap();

    env.command("alfa")
        .assert()
        .success()
        .stdout("alfa system\n");

    env._use("alfa", "1")
        .assert()
        .success();

    env.command("alfa")
        .assert()
        .success()
        .stdout("alfa1\n");
}
