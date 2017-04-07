extern crate clap;
use clap::App;

pub fn cli() -> Options {
    let yml = load_yaml!("cli.yml");
    App::from_yaml(yml).get_matches();
    Options
}

pub struct Options;
