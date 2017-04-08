extern crate reekup;

use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

use std::process::Command;

#[test]
fn help_command_works() {
    let output = Command::new("./target/debug/reekup")
        .arg("-h")
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("USAGE"));
}

#[test]
fn version_command_works() {
    let output = Command::new("./target/debug/reekup")
        .arg("-V")
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("reekup"));
}

#[test]
fn help_mentions_out_flag() {
    let output = Command::new("./target/debug/reekup")
        .arg("-h")
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--out"));
}

#[test]
fn help_mentions_url_flag() {
    let output = Command::new("./target/debug/reekup")
        .arg("-h")
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--url"));
}

#[test]
fn it_creates_a_config_reek_file_if_out_flag_not_provided() {
    assert!(File::open("config.reek").is_err());
    Command::new("./target/debug/reekup")
        .output()
        .expect("failed to execute");
    assert!(File::open("config.reek").is_ok());

    cleanup("config.reek");
}

#[test]
fn it_creates_a_the_provided_file_when_out_flag_passed() {
    assert!(File::open("created_with_out.reek").is_err());
    Command::new("./target/debug/reekup")
        .args(&["--out", "created_with_out.reek"])
        .output()
        .expect("failed to execute");
    assert!(File::open("created_with_out.reek").is_ok());

    cleanup("created_with_out.reek");
}

#[test]
fn running_twice_does_not_add_config_twice() {
    Command::new("./target/debug/reekup")
        .args(&["--out", "running_twice.reek"])
        .output()
        .expect("failed to execute");
    let file = File::open("running_twice.reek").unwrap();
    let line_count = BufReader::new(&file).lines().count();

    Command::new("./target/debug/reekup")
        .args(&["--out", "running_twice.reek"])
        .output()
        .expect("failed to execute");

    let updated_file = File::open("running_twice.reek").unwrap();
    assert_eq!(line_count, BufReader::new(&updated_file).lines().count());
    cleanup("running_twice.reek");
}

#[test]
fn custom_configs_are_not_lost() {
    Command::new("./target/debug/reekup")
        .args(&["--out", "custom_configs.reek"])
        .output()
        .expect("failed to execute");

    let mut file = File::create("custom_configs.reek").unwrap();
    write!(&mut file, "{}\n", "Custom Config").unwrap();

    Command::new("./target/debug/reekup")
        .args(&["--out", "custom_configs.reek"])
        .output()
        .expect("failed to execute");

    let updated_file = File::open("custom_configs.reek").unwrap();
    assert!(BufReader::new(&updated_file).lines().any(|l| l.unwrap() == "Custom Config"));
    cleanup("custom_configs.reek");
}

#[test]
fn standard_config_can_be_retrieved_from_a_url() {
    Command::new("./target/debug/reekup")
        .args(&["--out",
                "reek_defaults.reek",
                "--url",
                "https://raw.githubusercontent.\
                 com/troessner/reek/58bb1ff752ed68b6fd3bb5d56a4de43d14c8707a/defaults.reek"])
        .output()
        .expect("failed to execute");

    Command::new("./target/debug/reekup")
        .args(&["--out",
                "asr_defaults.reek",
                "--url",
                "https://raw.githubusercontent.com/umn-asr/dotfiles/master/reek"])
        .output()
        .expect("failed to execute");

    let reek_defaults = File::open("reek_defaults.reek").unwrap();
    let asr_defaults = File::open("asr_defaults.reek").unwrap();

    assert_ne!(BufReader::new(&reek_defaults).lines().count(),
               BufReader::new(&asr_defaults).lines().count());

    cleanup("reek_defaults.reek");
    cleanup("asr_defaults.reek");
}

fn cleanup(file_name: &str) {
    fs::remove_file(file_name).unwrap();
}
