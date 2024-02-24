# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
<!-- section:unreleased-body -->

### Changed

- Update dependencies

<!-- section:previous-releases -->
## [v1.5.0] 2023-10-29

### Added

- `alt scan` can now locate binaries from homebrew when it's installed in `/opt/homebrew`.
  Thank you [@Walrusse](https://github.com/Walrusse) for [#368](https://github.com/dotboris/alt/pull/368).

## [v1.4.0] 2023-05-31

### Added

- Add pre-built ARM binaries for Linux
- Add pre-built ARM binaries for MacOS

## [v1.3.1] 2023-01-08

### Changed

- Update `rand_core@0.6.0` crate to `0.6.4`. The previous version we were using
  has been yanked from `crates.io`.

## [v1.3.0] 2023-01-08

### Added

- Add man pages. Users will now be able to run `man alt` as well as `man
  alt-{subcommand}` in order to get documentation `alt` and its subcommands.

### Changed

- Migrate to Rust 2021 Edition. This should have no effect on users but may
  affect people building from source.
- Migrate to `clap@v4`. This changes the way the help text is rendered. In terms
  of behavior, nothing should change.
- Change some of the errors management to use the
  [`anyhow` crate](https://crates.io/crates/anyhow). This should make the errors
  affected by this change clearer and give more context.
- Rewrite how command versions are managed internally. This is a purely internal
  change and should not affect users in any way.

### Fixed

- Ensure `alt def` always stores the binary path it is given as an absolute
  path. This fixes an issue where `alt` would fail to run a command version if
  it was defined using a relative path.
  ([#274](https://github.com/dotboris/alt/issues/274))

## [v1.2.1] 2022-06-24

### Changed

- Rename default branch from `master` to `main`. This should have no effect on
  users but may affect people building from source.

### Fixed

- Fixed typo in help text for the `alt use` command.

## [v1.2.0] - 2020-12-06

### Added

- Added `alt doctor` command that can find and detect certain common problems
  with `alt` and sometimes automatically fix them. This first version of `alt
  doctor` currently only knowns how to find and fix broken command version
  definitions.

### Changed

- Display relevant debugging information when `alt` fails to execute a command.
  This applies for both the `alt exec` command as well as executing commands
  through shims (the usual way of running commands through `alt`). This should
  help people figure out what's going on when `alt` fails. Note that previously
  `alt` only displayed the generic underlying error.

## [v1.1.1] - 2019-11-16

### Added

- Document how to pass flags parsed by `alt exec` to the executed command
  instead of `alt exec` itself. This was done directly in `alt exec`'s help
  text.

### Removed

- Remove the `--version` & `-V` flags from all subcommands. This means that
  `alt --version` works just fine but `alt scan --version` does not. This was
  done because the `--version` flag on subcommands did not output anything
  useful.

### Changed

- Reduce final binary size by removing unused unicode regex features.
- Update dependencies.

### Fixed

- Fix bug where `alt scan` would not find single letter commands with a version
  suffix. Example: `a2`, `a-2`, `a2.2` `a-2.2`.

## [v1.1.0] - 2019-10-27

### Added

- `alt scan` now knows about LinuxBrew and can automatically find different
  version of commands installed through LinuxBrew.
- Add install instructions for Mac OSX using the Homebrew package manager.
  (LinuxBrew also supported)
- Add install instructions for `DEB` based linux systems.
- Add install instructions using the pre-built `.tar.gz` release.
- Add install from source instructions.
- Document how to troubleshoot the warning emitted by `alt` when the shim
  directory is not present in the `PATH` environment variable.
- Add Homebrew formula update to release instructions.

### Changed

- Update dependencies
- Move the shims directory higher up in the `PATH` on fish by using
  `fish_user_paths`.
- Expand documentation on `alt` in relation to git (and other VCS).
- Update link to rust install instructions.
- Update the shims dir not in `PATH` warning to include troubleshooting steps
  and to link to the troubleshooting documentation.

### Removed

- The gziped `alt` binary (`alt_{...}.gz`) is no longer packaged.
- Remove support for the `curl ... | bash -s` install method.

### Fixed

- Fix crash when `alt` is run without the `PATH` environment variable set.

<!-- section:refs -->
[v1.5.0]: https://github.com/dotboris/alt/compare/v1.4.0..v1.5.0
[v1.4.0]: https://github.com/dotboris/alt/compare/v1.3.1..v1.4.0
[v1.3.1]: https://github.com/dotboris/alt/compare/v1.3.0..v1.3.1
[v1.3.0]: https://github.com/dotboris/alt/compare/v1.2.1..v1.3.0
[v1.2.1]: https://github.com/dotboris/alt/compare/v1.2.0..v1.2.1
[v1.2.0]: https://github.com/dotboris/alt/compare/v1.1.1..v1.2.0
[v1.1.1]: https://github.com/dotboris/alt/compare/v1.1.0..v1.1.1
[v1.1.0]: https://github.com/dotboris/alt/releases/tag/v1.1.0
