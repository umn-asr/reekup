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

If you run reekup more than once it'll just keep appending to your config.reek file. So...don't do that.
