use clap::App;

use reekup::options::Options;

pub fn cli() -> Options {
    let yml = load_yaml!("cli.yml");
    App::from_yaml(yml).get_matches();
    Options::from_defaults()
}
