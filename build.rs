use clap_complete::{generate_to, shells::Bash, shells::Fish, shells::Zsh};
use clap_mangen::Man;
use std::boxed::Box;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::{env, io};

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(&out_dir);
    let out_dir = out_dir
        .ancestors() // .../target/<debug|release>/build/example-<SHA>/out
        .nth(3)
        .ok_or("failed to resolve debug or release dir in out dir")?; // .../target/<debug|release>

    let completion_dir = out_dir.join("completion");
    if !completion_dir.exists() {
        fs::create_dir_all(&completion_dir)?;
    }

    let man_dir = out_dir.join("man");
    if !man_dir.exists() {
        fs::create_dir_all(&man_dir)?;
    }

    let mut app = make_app();
    app.build();

    generate_to(Bash, &mut app, "alt", &completion_dir)?;
    generate_to(Zsh, &mut app, "alt", &completion_dir)?;
    generate_to(Fish, &mut app, "alt", &completion_dir)?;

    generate_manpage(app.clone(), &man_dir.join("alt.1"))?;
    for subcommand in app.get_subcommands() {
        generate_manpage(
            subcommand.clone(),
            &man_dir.join(format!("alt-{}.1", subcommand.get_name())),
        )?;
    }

    Ok(())
}

fn generate_manpage(app: Command, out_path: &Path) -> Result<(), io::Error> {
    let man = Man::new(app);

    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    fs::write(out_path, buffer)?;

    Ok(())
}
