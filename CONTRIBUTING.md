# Contribution Guide

## Cheat sheet

```sh
# build
cargo build

# run a local version of alt
cargo run ...

# test
cargo test

# format
cargo fmt

# lint
cargo clippy
```

## Setup

This project is built with rust. You'll need to have rust installed.
See: <https://www.rust-lang.org/tools/install>

## Snapshot tests

Some of the tests in this projects are snapshot based. Instead of asserting on
hard coded values, these tests use previously stored snapshots to ensure that
certain important values don't change.

These tests are built using the [`insta`](https://crates.io/crates/insta) crate.
You don't need any special tools to run these tests.

If you need to change these tests or break them and need to change them. Please
see <https://insta.rs/docs/quickstart/> for a quick guide on how to use them.
You'll mostly likely want to install the
[`cargo-insta`](https://crates.io/crates/cargo-insta) CLI which helps with
managing snapshots.
