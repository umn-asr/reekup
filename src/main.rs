extern crate reekup;

#[macro_use]
extern crate clap;

mod cli;
use cli::cli;

use reekup::options::Options;

fn main() {
    let c = cli();
    reekup::run(&Options::from_defaults());
}
