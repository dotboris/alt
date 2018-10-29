#![cfg(not(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    feature = "travis_ci"
)))]
// Long story short: the integration tests don't work and will not work
// in travis-ci on arm.
//
// Long story long:
// These tests work by executing the `alt` binary for the current
// release & target. This is done in two ways:
// - Direct call with the Command module
// - Indirect call through a shim (symlink pointing to the alt binary)
//
// This is usually fine since you tend to build binaries for the os you
// currently run. When building arm on travis, things break down.
// We're building arm binaries on a x64 machine that we can't run directly.
// `cross` runs arm binaries through `qemu-arm` (QEMU user mode).
// I think that it's possible to setup QEMU user mode to work seemlessly on
// linux, but I'm not going to bother with it for now.

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
