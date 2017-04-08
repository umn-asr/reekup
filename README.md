# Reekup

Creates or updates a project's [Reek configuration file](https://github.com/troessner/reek) with the current [ASR default](https://github.com/umn-asr/dotfiles/blob/master/reek).

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

`reekup` will append [ASR's default Reek configuration](https://github.com/umn-asr/dotfiles/blob/master/reek) to a `config.reek` file in the current directory.

If no `config.reek` file exists, it will be created.

If a `config.reek` file already exists, the contents of it will be preserved.

You can run `reekup` multiple times in the same project, keeping your `config.reek` file up to date with ASR's default.

`reekup -h` will show you the help.

| Flag        | Use                                                                                                                           |
| ----        | ----                                                                                                                          |
| -o or --out | file where your reek config will be stored. Defaults to config.reek                                                           |
| -u or --url | urr where your default reek config lives. Defaults to the [ASR default](https://github.com/umn-asr/dotfiles/blob/master/reek) |

## Development

- [Install Rust](https://www.rust-lang.org/en-US/)
- Clone this repo
- `cd reekup`
- `./script/setup`
- `./script/test`

© 2017 Regents of the University of Minnesota. All rights reserved.
