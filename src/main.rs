extern crate reekup;

#[macro_use]
extern crate clap;

mod cli;
mod options;
use cli::cli;

fn main() {
    let c = cli();
    reekup::run();
}
