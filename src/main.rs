extern crate reekup;

#[macro_use]
extern crate clap;

mod cli;
use cli::cli;

fn main() {
    reekup::run(&cli());
}
