extern crate reekup;

use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

#[test]
fn it_creates_a_config_reek_file() {
    assert!(File::open("config.reek").is_err());
    reekup::run();
    assert!(File::open("config.reek").is_ok());

    cleanup();
}

#[test]
fn running_twice_does_not_add_config_twice() {
    reekup::run();
    let file = File::open("config.reek").unwrap();
    let line_count = BufReader::new(&file).lines().count();

    reekup::run();

    let updated_file = File::open("config.reek").unwrap();
    assert_eq!(line_count, BufReader::new(&updated_file).lines().count());

    cleanup();
}

#[test]
fn custom_contents_are_not_lost() {
    reekup::run();

    let mut file = File::create("config.reek").unwrap();
    write!(&mut file, "{}\n", "Custom Config").unwrap();

    reekup::run();

    let updated_file = File::open("config.reek").unwrap();

    assert!(BufReader::new(&updated_file).lines().any(|l| l.unwrap() == "Custom Config"));

    cleanup();
}

fn cleanup() {
    fs::remove_file("config.reek").unwrap();
}
