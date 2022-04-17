use clap_complete::{generate_to, shells::Bash, shells::Fish, shells::Zsh};
use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::boxed::Box;
use std::fs;

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let completion_dir = PathBuf::from(&out_dir);
    let mut completion_dir = completion_dir
        .ancestors()  // .../target/<debug|release>/build/example-<SHA>/out
        .nth(3) // .../target/<debug|release>
        .unwrap()
        .to_owned();

    completion_dir.push("completion");

    if !completion_dir.exists() {
        fs::create_dir_all(&completion_dir)?;
    }

    let mut app = make_app();
    generate_to(Bash, &mut app, "alt", &completion_dir)?;
    generate_to(Zsh, &mut app, "alt", &completion_dir)?;
    generate_to(Fish, &mut app, "alt", &completion_dir)?;

    Ok(())
}
