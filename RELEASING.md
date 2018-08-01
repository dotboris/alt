# Releasing

## Requirements

- A working rust setup
  (https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
- A linux machine
- An OSX machine

## Version bump

1.  Update version in `Cargo.toml`
1.  Update `Cargo.lock`

    ```sh
    cargo build
    ```

1.  Commit your version bump

    ```sh
    git add .
    git commit -m 'bump version to v{your version number}'
    ```

1.  Tag a new version

    ```sh
    git tag v{your-version-number}
    ```

1.  Push everything

    ```sh
    git push --tags origin master
    ```

## Build release versions

You'll need to do the following steps twice. Once on a mac and once on linux.

1.  Checkout the right tag

    ```sh
    git checkout v{your version number}
    ```

1.  Build the release version

    ```sh
    ./build/build-release.sh {osx or linux}
    ```

1.  Push your binary to the github release page

    ```sh
    hub release edit -a dist/release/atl_{osx or linux} v{your version number}
    ```
