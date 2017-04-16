extern crate clap;
extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use self::yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct ConfigFile {
    pub target_filepath: String,
    pub source_url: String,
}

impl ConfigFile {
    pub fn from_args(args: &clap::ArgMatches) -> Option<Self> {
        if let Some(config_file) = args.value_of("config") {
            let x = File::open(&config_file);

            if x.is_err() {
                panic!("Config file does not exist")
            } else {
                let mut f = File::open(&config_file).unwrap();
                let mut buffer = String::new();
                f.read_to_string(&mut buffer).unwrap();
                let conf = YamlLoader::load_from_str(&buffer).unwrap();

                let doc = &conf[0];

                let target = String::from(doc["out"].as_str().unwrap());
                let url = String::from(doc["url"].as_str().unwrap());

                Some(ConfigFile {
                    target_filepath: target,
                    source_url: url,
                })
            }
        } else {
            None
        }
    }
}
