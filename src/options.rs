extern crate clap;
// use self::clap::ArgMatches;

#[derive(Debug)]
pub struct Options {
    pub target_filepath: String,
    pub source_url: String,
}

impl Options {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        Options {
            target_filepath: String::from(args.value_of("out").unwrap()),
            source_url: String::from(args.value_of("url").unwrap()),
        }
    }
}
