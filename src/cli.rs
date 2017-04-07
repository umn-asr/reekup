use clap::App;

use options::Options;

pub fn cli() -> Options {
    let yml = load_yaml!("cli.yml");
    App::from_yaml(yml).get_matches();
    Options::from_defaults()
}
