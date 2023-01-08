use assert_cmd::prelude::*;
mod test_env;
use std::fs;
use std::io::Result as IoResult;
use test_env::TestEnv;

fn def_all(env: &TestEnv) -> IoResult<()> {
    for command in &["alfa", "bravo", "charlie"] {
        env.create_stub_command(command, &format!("{} system version", command))?;

        for version in &["1", "2", "3"] {
            let stub_path = env.create_stub_command(
                &format!("{}{}", command, version),
                &format!("{} version {}", command, version),
            )?;

            env.def(command, version, &stub_path).assert().success();
        }
    }

    Ok(())
}

#[test]
fn def_and_use() -> IoResult<()> {
    let env = TestEnv::new();
    def_all(&env)?;

    env._use("alfa", "1").assert().success();

    env.command("alfa")
        .assert()
        .success()
        .stdout("alfa version 1");

    Ok(())
}

#[test]
fn system_with_no_use() -> IoResult<()> {
    let env = TestEnv::new();
    def_all(&env)?;

    env.command("bravo")
        .assert()
        .success()
        .stdout("bravo system version");

    Ok(())
}

#[test]
fn reset_with_use_system() -> IoResult<()> {
    let env = TestEnv::new();
    def_all(&env)?;

    env._use("charlie", "3").assert().success();

    env.command("charlie")
        .assert()
        .success()
        .stdout("charlie version 3");

    env._use("charlie", "system").assert().success();

    env.command("charlie")
        .assert()
        .success()
        .stdout("charlie system version");

    Ok(())
}

#[test]
fn use_with_subdir() -> IoResult<()> {
    let env = TestEnv::new();
    def_all(&env)?;

    env._use("alfa", "3").assert().success();

    let subdir = env.root.join("subdir");
    fs::create_dir(&subdir).unwrap();
    env.command("alfa")
        .current_dir(&subdir)
        .assert()
        .success()
        .stdout("alfa version 3");

    Ok(())
}

#[test]
fn use_with_subdir_overwrite() -> IoResult<()> {
    let env = TestEnv::new();
    def_all(&env)?;

    env._use("bravo", "1").assert().success();

    env.command("bravo")
        .assert()
        .success()
        .stdout("bravo version 1");

    let subdir = env.root.join("subdir");
    fs::create_dir(&subdir).unwrap();

    env._use("bravo", "2").assert().success();

    env.command("bravo")
        .current_dir(&subdir)
        .assert()
        .success()
        .stdout("bravo version 2");

    Ok(())
}

#[test]
fn def_with_relative_path_works_after_dir_change() -> anyhow::Result<()> {
    let env = TestEnv::new();

    let stub_command_path = env.create_stub_command("foo-42", "this is foo@42")?;

    // Define foo@42 using a relative path. The `TestEnv` manages a working
    // directory under such that all commands run through it run in that
    // directory. The relative path is relative to that directory.
    let relative_path = stub_command_path.strip_prefix(&env.root)?;
    println!("{:?}", relative_path);
    assert!(relative_path.is_relative());
    env.def("foo", "42", relative_path).assert().success();

    env._use("foo", "42").assert().success();

    env.command("foo")
        .assert()
        .success()
        .stdout("this is foo@42");

    fs::create_dir(env.root.join("subdir"))?;
    env.command("foo")
        .assert()
        .success()
        .stdout("this is foo@42");

    Ok(())
}
