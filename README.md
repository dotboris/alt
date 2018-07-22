# alt

:rotating_light: this project is a work in progress :rotating_light:

Command to switch between different versions of commands.

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

`alt` looks for commands and their versions in the `~/.config/alt/defs.toml`
file. Open that file with your text editor of choice.
(create it if it's not already there)

Here's an example of a command definition file:

```toml
# A section defines a command
# Here we define the python command
[python]
# Under a section we associate a version name (in this case 2 and 3) to executable.
# You can name versions whatever you want. We use version numbers because it's simple
2 = "/usr/bin/python2"
3 = "/usr/bin/python3"

# Another section defining the node command
[node]
4 = "/path/to/node4/bin/node"
6 = "/path/to/node6/bin/node"
8 = "/path/to/node8/bin/node"
```

When you make changes to the command definitions, remember to generate the shims.

```sh
alt shim
```

### Switching command versions

Currently, `alt` only supports switching versions of commands based on the
current directory. The idea is to let you define which versions of a command you
want to use for your project.

1.  Go to the root of a directory tree where you want to use a different command

    ```sh
    cd path/to/your/project
    ```

1.  Create a file named `.alt.toml`

    ```toml
    # Associate a command to a version defined in the command definition file
    node = "8"
    python = "3"
    ```

## Development

TODO: development doc

## Relase

TODO: Release doc
