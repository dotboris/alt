# Releasing

1.  Update version in `Cargo.toml`
1.  Update `Cargo.lock`

    ```sh
    cargo build
    ```

1.  Add the new version to the `CHANGELOG.md`

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
    git push --tags origin main
    ```

1.  Travis CI will build the release and push it to github
1.  Wait for Travis CI to finish building & pushing the artifacts
1.  Update the Homebrew formula in <https://github.com/dotboris/homebrew-alt>
