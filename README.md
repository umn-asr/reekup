# Reekup

Creates or updates a project's [Reek configuration file](https://github.com/troessner/reek) with the current [ASR standard](https://github.com/umn-asr/dotfiles/blob/master/reek).

## Installation

To get a pre-compiled binary (Mac only), you can use [Homebrew](https://brew.sh/)

- `brew tap umn-asr/homebrew-tap`
- `brew install umn-asr/homebrew-tap/reekup`

Otherwise, you can compile it yourself

- [Install Rust](https://www.rust-lang.org/en-US/)
- Clone this repo
- `cd reekup`
- `cargo build --release`
- `cp ./target/release/reekup [somewhere in your $PATH]`

## Usage

`reekup` will append the latest Reek configuration to a `config.reek` file in the current directory.

If no `config.reek` file exists, it will be created.

If a `config.reek` file already exists, the contents of it will be preserved.

You can run `reekup` multiple times in the same project, keeping your `config.reek` file up to date with ASR's standard.

## Development

- [Install Rust](https://www.rust-lang.org/en-US/)
- Clone this repo
- `cd reekup`
- `./script/setup`
- `./script/test`
    - Note, tests are run with `RUST_TEST_THREADS=1` to remove parallelization.
    - Running the `tests/reekup.rs` tests in parallel leads to random failure due to the tests all trying to write to & delete the same file.
