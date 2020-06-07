# roadrunner ![Rust](https://github.com/roadrunner-craft/client/workflows/Rust/badge.svg)

Yet another blocky game :)

## Dependencies

You'll need the most up to date version of [core](https://github.com/roadrunner-craft/core), [math](https://github.com/roadrunner-craft/math) and [assets](https://github.com/roadrunner-craft/assets)

#### Export the assets

```sh
./assets/scripts/export.py ./client
```

## Run

```sh
cargo run [--release]
```

## Build

```sh
# install the bundle command
cargo install cargo-bundle

# build for a release
cargo bundle --release
```
## Features

To enable a feature, type `cargo run --features FEATURE_NAME`. Here's the list of currently available features:

- watchers: watch the res folder to reload assets at runtime
- remote: temporary flag to make the client connect to the server hardcoded in `main.rs`
