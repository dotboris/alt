# Install Pre-packaged Binaries

Note that the following instructions assume a fairly "normal" file system.
You may need to adjust some of the paths here to match your system.

1.  Open the [latest release page on Github][latest-release]
1.  Download the `.tar.gz` file that matches your system architecture
1.  Extract the `.tar.gz` file

    Before you run this, be sure to replace `{version}` & `{system}` to match
    the file you downloaded.

    ```sh
    tar xvzf alt_{version}_{system}.tar.gz
    cd alt_{version}_{system}
    ```

1.  Install the `alt` binary

    ```sh
    sudo cp bin/alt /usr/local/bin/alt
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
    sudo cp completion/alt.bash /etc/bash_completion.d/alt.bash
    # If you use ZSH
    sudo cp completion/_alt /usr/share/zsh/site-functions/_alt
    # If you use FISH
    sudo cp completion/alt.fish /etc/fish/completions/alt.fish
    ```

You will probably need to log out & log back in to your user so that the `PATH`
configuration scripts you install will load.

[latest-release]: https://github.com/dotboris/alt/releases/latest
