# Install from Source

Note that the following instructions assume a fairly "normal" file system. You
may need to adjust some of the paths here to match your system.

1.  Open the [latest release page on Github][latest-release]
1.  Download the source `.tar.gz` file
1.  Install the Rust compiler

    See: <https://www.rust-lang.org/tools/install>

1.  Extract the source code

    Before you run this, be sure to replace `{version}` to match the file you
    downloaded.

    ```sh
    tar xvzf {version}.tar.gz
    cd alt-{version}
    ```

1.  Compile `alt` (This will take a while)

    ```sh
    cargo build --release --locked
    ```

1.  Install the `alt` binary

    ```sh
    sudo cp target/release/alt /usr/local/bin/alt
    ```

1.  Install the `PATH` configuration scripts

    ```sh
    sudo cp etc/profile.d/alt.sh /etc/profile.d/alt.sh
    # (Optional) if you use fish
    sudo cp etc/fish/conf.d/alt.fish /etc/fish/conf.d/alt.fish
    ```

1.  (Optional) Install the completions

    ```sh
    # If you use BASH
    sudo cp target/release/completion/alt.bash /etc/bash_completion.d/alt.bash
    # If you use ZSH
    sudo cp target/release/completion/_alt /usr/share/zsh/site-functions/_alt
    # If you use FISH
    sudo cp target/release/completion/alt.fish /etc/fish/completions/alt.fish
    ```

1.  (Optional) Install man pages

    ```sh
    sudo mkdir -p /usr/local/share/man/man1
    sudo cp target/release/man/* /usr/local/share/man/man1/
    ```

You will probably need to log out & log back in to your desktop session so that
the `PATH` configuration scripts you installed will load.

[latest-release]: https://github.com/dotboris/alt/releases/latest
