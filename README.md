# roadrunner ![Rust](https://github.com/roadrunner-craft/client/workflows/Rust/badge.svg)

Yet another blocky game :)

## Run

`cargo run [--release]`

## Build

    # install the bundle command
    cargo install cargo-bundle

    # build for a release
    cargo bundle --release

## Features

To enable a feature, type `cargo run --features FEATURE_NAME`. Here's the list of currently available features:

- watchers: watch the res folder to reload assets at runtime
- remote: temporary flag to make the client connect to the server hardcoded in `main.rs`
