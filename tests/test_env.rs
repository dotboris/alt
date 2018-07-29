extern crate rand;
extern crate assert_cmd;

use self::rand::prelude::*;
use self::rand::distributions::Alphanumeric;
use std::env;
use std::fs;
use std::path::*;
use std::process::Command;
use assert_cmd::prelude::*;

pub fn create_env() -> PathBuf {
    let rand_ns: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect();

    let dir = env::temp_dir()
        .join(format!("alt-tests-{}", rand_ns));

    fs::create_dir(&dir)
        .expect(&format!("failed to created tmp env {:?}", &dir));
    dir
}

pub fn delete_env(dir: PathBuf) {
    fs::remove_dir_all(&dir)
        .expect(&format!("failred to remove {:?}", &dir));
}

pub fn set_env<'a>(c: &'a mut Command, test_env: &Path) -> &'a mut Command {
    c.env("ALT_HOME", test_env.join("alt-home"));
    c.env("ALT_SHIM_DIR", test_env.join("shims"));
    c.env("PATH", test_env.join("shims"))
}

pub fn fixture_command_path(name: &str, version: &i32) -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("fixtures/commands")
        .join(format!("{}{}", name, version))
}
