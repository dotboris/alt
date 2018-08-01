# alt

[![Build Status](https://travis-ci.org/dotboris/alt.svg?branch=master)](https://travis-ci.org/dotboris/alt)

Tool for switching between different versions of commands.

## Installation

Currently, `alt` doesn't have a binary release. If you want to use `alt`, you'll
need to build it from source.

1.  Setup a rust development environment

    See: https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html

1.  Clone this repository

    ```sh
    git clone https://github.com/dotboris/alt.git
    cd alt
    ```

1.  Build & Install `alt` from source

    ```sh
    cargo install
    ```

1.  Check if `alt` is available

    ```sh
    alt --help
    ```

1.  Add the shims directory to your `PATH` environment variable

    ```sh
    # in ~/.zshrc or ~/.bashrc
    export PATH="$HOME/.local/alt/shims:$PATH"
    ```

## Usage

In order to use `alt`, there are two things that you need to do:

1.  Tell `alt` about the commands on your system and their different versions
1.  Tell `alt` what version of a given command you want to use in a directory

### Defining commands and versions

`alt` can automatically scan your system to find different versions of command.

```sh
alt scan some-command
```

Currently, scanning supports:

- Looking through `PATH` for commands with version suffixes
- Looking through homebrew versioned packages (ex: `node@8`)

If `alt` can't find your command versions automatically, you can define them
by hand with:

```sh
alt def some-command version-name /path/to/command/bin
```

### Switching command versions

`alt` uses the directory you're currently in to figure out which versions of
commands to run. When switching versions, everything is related to the current
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
