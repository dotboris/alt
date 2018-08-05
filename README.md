# alt the version switcher

[![Build Status](https://travis-ci.org/dotboris/alt.svg?branch=master)](https://travis-ci.org/dotboris/alt)

Tool for switching between different versions of commands.

[![Screencast demo of alt](https://asciinema.org/a/5X4W5GEEMmBjlNl84yaAehnmh.png)](https://asciinema.org/a/5X4W5GEEMmBjlNl84yaAehnmh?autoplay=1)

## Installation

1.  Install alt

    ```sh
    curl -sL https://github.com/dotboris/alt/raw/master/install.sh | sh -s
    ```

1.  Add the shims directory to your `PATH` environment variable

    For BASH:

    ```sh
    echo 'export PATH="$HOME/.local/alt/shims:$PATH"' >> ~/.bashrc
    ```

    For ZSH:

    ```sh
    echo 'export PATH="$HOME/.local/alt/shims:$PATH"' >> ~/.zshrc
    ```

    For FISH:

    ```sh
    echo 'set -x PATH "$HOME/.local/alt/shims" $PATH' >> ~/.config/fish/config.fish
    ```

## Usage

### Define command versions

Automatically

```sh
alt scan some-command
```

or manually

```sh
alt def some-command version-name /path/to/command/bin
```

### Switch command version

It's important to understand that that __`alt` works with the current
directory__. When you switch command versions, you do so for the current
directory.

```sh
cd directory/of/interest
alt use some-command
```

## Development

### Setup

See: https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html

### Run

```sh
cargo run ...
```

### Test

```sh
cargo test
```
