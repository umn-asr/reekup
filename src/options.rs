pub struct Options {
    pub target_filepath: String,
    pub source_url: String,
}

impl Options {
    pub fn from_defaults() -> Self {
        Options {
            target_filepath: String::from("config.reek"),
            source_url: String::from("https://raw.githubusercontent.\
                                      com/umn-asr/dotfiles/master/reek"),
        }
    }
}
