#[macro_use]
extern crate clap;
use clap::Shell;
use std::env;
use std::path::PathBuf;
use std::fs;

include!("src/cli.rs");

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let completion_dir = PathBuf::from(&out_dir);
    let mut completion_dir = completion_dir
        .ancestors()  // .../target/<debug|release>/build/example-<SHA>/out
        .skip(3)      // .../target/<debug|release>
        .next().unwrap().to_owned();
    completion_dir.push("completion");

    fs::create_dir_all(&completion_dir).unwrap();
    let mut app = make_app();
    app.gen_completions("alt", Shell::Bash, &completion_dir);
    app.gen_completions("alt", Shell::Zsh, &completion_dir);
    app.gen_completions("alt", Shell::Fish, &completion_dir);
}
