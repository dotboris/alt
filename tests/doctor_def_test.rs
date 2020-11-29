extern crate assert_cmd;

use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_env;
use test_env::TestEnv;
use std::fs;
use std::io::Result as IoResult;

#[test]
fn remove_entry_for_missing_bin() -> IoResult<()> {
    let env = TestEnv::new();

    env.create_stub_command("thingy", "this is thingy system version")?;
    let bin_v1_path = env.create_stub_command(
        "thingy-1",
        "this is thingy v1"
    )?;
    let bin_v2_path = env.create_stub_command(
        "thingy-2",
        "this is thingy v2"
    )?;

    env.def("thingy", "1", &bin_v1_path).assert().success();
    env.def("thingy", "2", &bin_v2_path).assert().success();

    fs::remove_file(bin_v1_path)?;

    env.alt()
        .arg("show")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("thingy-1")
                .and(predicate::str::contains("thingy-2"))
        );

    env.alt()
        .args(&["doctor", "--fix-mode", "auto"])
        .spawn()?
        .wait()?;

    env.alt()
        .arg("show")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("thingy-1").not()
                .and(predicate::str::contains("thingy-2"))
        );

    Ok(())
}
