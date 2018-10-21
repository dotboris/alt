# alt the version switcher

[![Build Status](https://travis-ci.org/dotboris/alt.svg?branch=master)](https://travis-ci.org/dotboris/alt)

`alt` is a command line utility that lets you switch between different versions
of commands based on your current directory.

<p align="center">
  <a href="https://asciinema.org/a/195103?autoplay=1" target="_blank">
    <img alt="Screencast demo of alt" src="demo.gif" />
  </a>
</p>

## Why?

As developers, we work with a large number of tools. When we move from codebase
to codebase, those tools and their versions change. Switching between the
different versions of those tools every time you change codebase is a nightmare.

This is where `alt` comes in. It will automatically switch the version of
commands when you move to a different codebase.

There are other tools out there that solve this problem. `alt` distinguish
itself in a few ways:

-   __tool / language agnostic__: Some version switching tools only work with a
    specific tool or programming language. `alt` is generic. It works for any
    command.
-   __no shell pollution__: Most version switching tools hook themselves into
    your shell. This can slow down your shell's start time. `alt` does not hook
    into your shell. You can use it without slowing down your shell start time.
-   __only version switching__: Unlike other tools, `alt` does not take
    responsibility for installing different versions of commands or managing
    their dependencies. How you install install different versions of commands
    is entirely up to you.

## Installation

1.  Install the `alt` binary

    ```sh
    curl -sL https://github.com/dotboris/alt/raw/master/install.sh | bash -s
    ```

1.  Configure your `PATH` to let `alt` change command versions

    For BASH:

    ```sh
    echo 'export PATH="$HOME/.local/alt/shims:$PATH"' >> ~/.bashrc
    export PATH="$HOME/.local/alt/shims:$PATH"
    ```

    For ZSH:

    ```sh
    echo 'export PATH="$HOME/.local/alt/shims:$PATH"' >> ~/.zshrc
    export PATH="$HOME/.local/alt/shims:$PATH"
    ```

    For FISH:

    ```sh
    echo 'set -x PATH "$HOME/.local/alt/shims" $PATH' >> ~/.config/fish/config.fish
    set -x PATH "$HOME/.local/alt/shims" $PATH
    ```

1.  (Optional) Add `.alt.toml` to your global gitignore file

    During it's operation, `alt` puts a file named `.alt.toml` in the current
    directory. These files don't belong in git repositories. To avoid getting
    those files all over your git repositories, you can add them to a global
    gitignore file.

    If you don't know how to create a global gitignore file, see:
    https://help.github.com/articles/ignoring-files/#create-a-global-gitignore

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
