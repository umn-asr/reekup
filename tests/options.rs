extern crate reekup;

use reekup::options::Options;

#[test]
fn defaults_to_a_target_of_config_reek() {
    let subject = Options::from_defaults();
    assert_eq!("config.reek", subject.target_filepath)
}

#[test]
fn defaults_to_asr_config_as_source() {
    let subject = Options::from_defaults();
    assert_eq!("https://raw.githubusercontent.com/umn-asr/dotfiles/master/reek",
               subject.source_url)
}
