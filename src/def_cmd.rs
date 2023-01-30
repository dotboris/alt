use crate::command_version::CommandVersion;
use crate::environment::load_command_version_registry;
use crate::shim;
use anyhow::Context;
use std::env;
use std::fs;
use std::path::*;

pub fn run(command: &str, version: &str, bin: &str) -> anyhow::Result<()> {
    let bin_path = fs::canonicalize(Path::new(bin))
        .with_context(|| format!("failed to resolve {bin} to an absolute path"))?;

    let mut registry = load_command_version_registry()?;
    registry.add(CommandVersion::new(command, version, &bin_path))?;
    registry
        .save()
        .context("failed to save command version registry file")?;

    shim::make_shim(command, env::current_exe().unwrap().as_path())
        .unwrap_or_else(|err| panic!("failed to create shim for {command}: {err}"));

    Ok(())
}
