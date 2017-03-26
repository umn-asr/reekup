extern crate reekup;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();
    reekup::run();
}
