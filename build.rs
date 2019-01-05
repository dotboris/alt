#[macro_use]
extern crate clap;
use clap::Shell;
use std::env;

include!("src/cli.rs");

pub fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = make_app();
    app.gen_completions("alt", Shell::Bash, &outdir);
    app.gen_completions("alt", Shell::Zsh, &outdir);
    app.gen_completions("alt", Shell::Fish, &outdir);
}
