extern crate clap;
extern crate yaml_rust;

use config_file::ConfigFile;

#[derive(Debug)]
pub struct Options {
    pub target_filepath: String,
    pub source_url: String,
}

impl Options {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        if let Some(config_file) = ConfigFile::from_args(&args) {
            Options {
                target_filepath: config_file.target_filepath,
                source_url: config_file.source_url,
            }
        } else {
            Options {
                target_filepath: String::from(args.value_of("out").unwrap()),
                source_url: String::from(args.value_of("url").unwrap()),
            }
        }
    }
}
