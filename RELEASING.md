# Releasing

## Requirements

- A linux machine with rust
- An OSX machine with rust

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

1.  Create a release

    Go to https://github.com/dotboris/alt/releases and create a draft release
    for the tag you just created.

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

    Open the release you created and attach the binary in `dist/relase/`.

## Publish the relase

When the release has all binaries in it, publish it
