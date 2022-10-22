#![cfg(test)]

mod test_env;
use clap::crate_version;
use serde::Serialize;
use std::{convert::TryFrom, error::Error, process::Output};
use test_case::test_case;
use test_env::TestEnv;

type TestResult = Result<(), Box<dyn Error>>;

#[derive(Debug, Serialize)]
struct OutputSnapshot {
    status: i32,
    stdout: String,
    stderr: String,
}

impl TryFrom<Output> for OutputSnapshot {
    type Error = Box<dyn Error>;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        let res = OutputSnapshot {
            status: value.status.code().ok_or("unable to read status code")?,
            stdout: String::from_utf8(value.stdout)?,
            stderr: String::from_utf8(value.stderr)?,
        };

        Ok(res)
    }
}

#[test_case(vec!["help"]; "help command")]
#[test_case(vec!["--help"]; "long help flag")]
#[test_case(vec!["-h"]; "short help flag")]
#[test_case(vec!["def", "--help"]; "def long help flag")]
#[test_case(vec!["def", "-h"]; "def short help flag")]
#[test_case(vec!["doctor", "--help"]; "doctor long help flag")]
#[test_case(vec!["doctor", "-h"]; "doctor short help flag")]
#[test_case(vec!["exec", "--help"]; "exec long help flag")]
#[test_case(vec!["exec", "-h"]; "exec short help flag")]
#[test_case(vec!["scan", "--help"]; "scan long help flag")]
#[test_case(vec!["scan", "-h"]; "scan short help flag")]
#[test_case(vec!["shim", "--help"]; "shim long help flag")]
#[test_case(vec!["shim", "-h"]; "shim short help flag")]
#[test_case(vec!["show", "--help"]; "show long help flag")]
#[test_case(vec!["show", "-h"]; "show short help flag")]
#[test_case(vec!["use", "--help"]; "use long help flag")]
#[test_case(vec!["use", "-h"]; "use short help flag")]
#[test_case(vec!["which", "--help"]; "which long help flag")]
#[test_case(vec!["which", "-h"]; "which short help flag")]
fn test_help(args: Vec<&str>) -> TestResult {
    let env = TestEnv::new();
    let output = env.alt().args(&args).output()?;

    let snapshot = OutputSnapshot::try_from(output)?;
    insta::with_settings!({
        filters => vec![(regex::escape(crate_version!()).as_str(), "[version]")],
        snapshot_suffix => args.join("_"),
    }, {
        insta::assert_toml_snapshot!(snapshot);
    });

    Ok(())
}
