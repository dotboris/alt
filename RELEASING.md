# Releasing

1.  Open the [Release workflow](https://github.com/dotboris/alt/actions/workflows/release.yml).
1.  Click on "Run workflow". A form will appear.
1.  Make sure that `main` is the selected branch.
1.  Choose what kind of version bump you want. You can look at
    [`CHANGELOG.md`](./CHANGELOG.md) to help you make that decision. Remember
    that this repository follows [Semantic Versioning](https://semver.org/).
1.  Click on the "Run workflow" button.
1.  This will start a new workflow run which will appear in the list
    momentarily. You can click on this new run to follow along.
1.  Wait for the workflow to complete succesfully.
1.  Update the Homebrew formula in <https://github.com/dotboris/homebrew-alt>.
