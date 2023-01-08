use crate::command_version::CommandVersion;
use crate::environment::load_command_version_registry;
use crate::shim;
use anyhow::Context;
use std::env;
use std::path::*;
use std::process;

pub fn run(command: &str, version: &str, bin: &str) -> anyhow::Result<()> {
    let bin_path = PathBuf::from(bin);

    if !bin_path.exists() {
        println!("File not found: {}", bin);
        process::exit(1);
    }

    let mut registry = load_command_version_registry()?;
    registry.add(CommandVersion::new(command, version, &bin_path))?;
    registry
        .save()
        .context("failed to save command version registry file")?;

    shim::make_shim(command, env::current_exe().unwrap().as_path())
        .unwrap_or_else(|err| panic!("failed to create shim for {}: {}", command, err));

    Ok(())
}
