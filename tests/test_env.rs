extern crate rand;
extern crate assert_cmd;
extern crate escargot;

use self::rand::prelude::*;
use self::rand::distributions::Alphanumeric;
use std::ffi::OsStr;
use std::env;
use std::fs;
use std::path::*;
use std::process::Command;
use self::escargot::CargoBuild;

#[derive(Debug)]
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

        let bin = CargoBuild::new()
            .bin("alt")
            .current_release()
            .current_target()
            .run()
            .unwrap();

        TestEnv {
            root: root,
            project_root: env::current_dir().unwrap(),
            alt_bin: PathBuf::from(bin.path())
        }
    }

    pub fn fixture_path(&self, command: &str, version: i32) -> PathBuf {
        self.project_root
            .join("fixtures/commands")
            .join(format!("{}{}", command, version))
    }

    pub fn command<T: AsRef<OsStr>>(&self, name: T) -> Command {
        let mut c = Command::new(name);
        c.current_dir(&self.root);
        c.env_clear();
        c.env("ALT_HOME", self.root.join("alt-home"));
        c.env("ALT_SHIM_DIR", self.root.join("shims"));
        c.env("PATH", env::join_paths(&[
            self.root.join("shims"),
            self.project_root.join("fixtures/system_commands")
        ]).unwrap());
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
            .expect(&format!("failed to remove {:?}", &self.root));
    }
}
