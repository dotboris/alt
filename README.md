# alt the version switcher

![GitHub All Releases](https://img.shields.io/github/downloads/dotboris/alt/total)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/dotboris/alt?sort=semver)
[![Build Status](https://travis-ci.org/dotboris/alt.svg?branch=master)](https://travis-ci.org/dotboris/alt)
![License](https://img.shields.io/github/license/dotboris/alt)

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
    their dependencies. How you install different versions of commands is
    entirely up to you.

## Installation

1.  Install the `alt` binary.

    ```sh
    curl -sL https://github.com/dotboris/alt/raw/master/install.sh | bash -s
    ```

1.  Add the `alt` shims directory to the top of your `PATH`.
    This lets `alt` change command versions.

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

1.  (Optional) Add `.alt.toml` to your global gitignore file.

    During it's operation, `alt` puts a file named `.alt.toml` in the current
    directory. These files don't belong in git repositories. To avoid getting
    those files all over your git repositories, you can add them to a global
    gitignore file.

    If you don't know how to create a global gitignore file, see:
    https://help.github.com/articles/ignoring-files/#create-a-global-gitignore

## Usage

Using `alt` is done in two steps:

1.  First, you tell `alt` about the different versions of commands installed on
    your system.
1.  Second, you tell `alt` what version of your commands to use in a given
    directory.

### Define command versions

`alt` can automatically scan your system to find different version of a command.
This can be done with the `alt scan` command:

```sh
alt scan some-command
```

This will bring up a menu that lets you choose all the versions of the given
command that you want to use with `alt`.

- <kbd>↑</kbd> / <kbd>↓</kbd> or <kbd>j</kbd> / <kbd>k</kbd>: Move cursor
- <kbd>Space</kbd>: Make version available to `alt`
- <kbd>Enter</kbd>: Confirm and save selection

If `alt` is not able to find a version of a command automatically for you, you
can always define the command version by hand.

This can be done with the `alt def` command:

```sh
alt def some-command version-name /path/to/command/bin
```

### Switch command version

Remember that `alt` decides what version of a command to use based on the
current directory. When you select a command version, it's for the current
directory.

You can tell `alt` to use a specific version of a command in the current
directory with the `alt use` command:

```sh
alt use some-command
```

This will bring up a menu that lets you choose the version of the specified
command that you want to use.

- <kbd>↑</kbd> / <kbd>↓</kbd> or <kbd>j</kbd> / <kbd>k</kbd>: Move cursor
- <kbd>Enter</kbd>: Select version to use

If menus aren't your cup of tea, you can specify the version on the command
line:

```sh
alt use some-command version-name
```

Note: If you want to use the system version without the menu, you can pass
`system` as the `version-name`.

```sh
alt use some-command system
```

### Show known commands & used versions

```sh
alt show
```

The above command will show you:

- All commands `alt` knows about
- The versions of those commands available
- The versions being used in the current directory

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
