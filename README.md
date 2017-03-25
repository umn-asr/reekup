# Reekup

Creates or updates a project's [Reek configuration file](https://github.com/troessner/reek) with the current ASR standard.

## Installation

To get a pre-compiled binary (Mac only), you can use [Homebrew](https://brew.sh/)

- `brew tap umn-asr/homebrew-tap`
- `brew install umn-asr/homebrew-tap/reekup`

Otherwise, you can compile it yourself

- [Install rust](https://www.rust-lang.org/en-US/)
- Clone this repo
- `cargo build --release`
- `cp ./target/release/reekup [somewhere in your $PATH]`

## Usage

`rustup` will append the latest Reek configuration to a `config.reek` file in the current directory.

If no `config.reek` file exists, it will be created.
