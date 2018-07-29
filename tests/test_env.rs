extern crate rand;
extern crate assert_cmd;

use self::rand::prelude::*;
use self::rand::distributions::Alphanumeric;
use std::ffi::OsStr;
use std::env;
use std::fs;
use std::path::*;
use std::process::Command;
use assert_cmd::cargo;

pub struct TestEnv {
    pub root: PathBuf,
    project_root: PathBuf,
    alt_bin: PathBuf,
}

impl TestEnv {
    pub fn new() -> Self {
        let rand_ns: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();

        let root = env::temp_dir()
            .join(format!("alt-tests-{}", rand_ns));

        fs::create_dir(&root)
            .expect(&format!("failed to created tmp env {:?}", &root));

        TestEnv {
            root: root,
            project_root: env::current_dir().unwrap(),
            alt_bin: cargo::main_binary_path().unwrap(),
        }
    }

    pub fn command<T: AsRef<OsStr>>(&self, name: T) -> Command {
        let mut c = Command::new(name);
        set_env(&mut c, self);
        c
    }

    pub fn alt(&self) -> Command {
        self.command(&self.alt_bin)
    }

    pub fn def(&self, command: &str, version: &str, path: &Path) -> Command {
        let mut c = self.alt();
        c.args(&["def", command, version, path.to_str().unwrap()]);
        c
    }

    pub fn _use(&self, command: &str, version: &str) -> Command {
        let mut c = self.alt();
        c.args(&["use", command, version]);
        c
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root)
            .expect(&format!("failred to remove {:?}", &self.root));
    }
}

fn set_env<'a>(c: &'a mut Command, test_env: &TestEnv) -> &'a mut Command {
    c.env("ALT_HOME", test_env.root.join("alt-home"));
    c.env("ALT_SHIM_DIR", test_env.root.join("shims"));
    c.env("PATH", env::join_paths(&[
        test_env.root.join("shims"),
        test_env.project_root.join("fixtures/system_commands")
    ]).unwrap())
}

pub fn fixture_command_path(name: &str, version: i32) -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("fixtures/commands")
        .join(format!("{}{}", name, version))
}
