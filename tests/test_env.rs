extern crate rand;
extern crate assert_cmd;
extern crate escargot;

use self::rand::prelude::*;
use self::rand::distributions::Alphanumeric;
use std::ffi::OsStr;
use std::env;
use std::io::{Write, BufWriter, Result as IoResult};
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::path::*;
use std::process::Command;
use self::escargot::CargoBuild;

#[derive(Debug)]
pub struct TestEnv {
    pub root: PathBuf,
    project_root: PathBuf,
    alt_bin: PathBuf,
    stub_bin_dir: PathBuf,
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
            .expect(&format!(
                "failed to created tmp env {}",
                root.display()
            ));

        let stub_bin_dir = root.join("stub-bins");
        fs::create_dir(&stub_bin_dir)
            .expect(&format!(
                "Failed to create directory for stub bins ({})",
                stub_bin_dir.display()
            ));

        let bin = CargoBuild::new()
            .bin("alt")
            .current_release()
            .current_target()
            .run()
            .unwrap();

        TestEnv {
            root: root,
            stub_bin_dir: stub_bin_dir,
            project_root: env::current_dir().unwrap(),
            alt_bin: PathBuf::from(bin.path()),
        }
    }

    pub fn create_stub_command(&self, command: &str, display_text: &str) -> IoResult<PathBuf> {
        let command_path = self.stub_bin_dir.join(command);

        let file = File::create(&command_path)?;

        let mut writer = BufWriter::new(&file);
        writeln!(&mut writer, "#!/bin/bash")?;
        writeln!(&mut writer, "echo -n '{}'", display_text)?;
        writer.flush()?;

        let mut perms = file.metadata()?
            .permissions();
        perms.set_mode(0o755);
        file.set_permissions(perms)?;

        Ok(command_path)
    }

    pub fn command<T: AsRef<OsStr>>(&self, name: T) -> Command {
        let mut c = Command::new(name);
        c.current_dir(&self.root);
        c.env_clear();
        c.env("ALT_HOME", self.root.join("alt-home"));
        c.env("ALT_SHIM_DIR", self.root.join("shims"));
        c.env("PATH", env::join_paths(&[
            self.root.join("shims"),
            self.stub_bin_dir.clone(),
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
