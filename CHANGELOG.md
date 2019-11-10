# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Reduce final binary size by removing unused unicode regex features.

### Fixed

- Fix bug where `alt scan` would not find single letter commands with a version
  suffix. Example: `a2`, `a-2`, `a2.2` `a-2.2`.

## v1.1.0 - 2019-10-27

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
