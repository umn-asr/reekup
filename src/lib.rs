extern crate curl;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs;
use std::fs::File;

use self::curl::easy::Easy;

use std::io::BufReader;
use std::io::BufRead;

pub fn run() {
    let reek_standards = get_reek_standards();
    match reek_standards {
        Ok(_) => update_config(),
        Err(_) => println!("Unable to make http request"),
    }
    fs::remove_file(".standard.reek").unwrap();
}

fn update_config() {
    let temp = OpenOptions::new()
        .write(true)
        .create(true)
        .open("reekup.tmp");

    let mut temp = match temp {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let current = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("config.reek");

    let current = match current {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let current_buffer = BufReader::new(&current);

    let mut copy_line = true;

    for line in current_buffer.lines() {
        let l = line.unwrap();

        if l == "### Reekup Begin" {
            copy_line = false;
        }

        if copy_line {
            write!(&mut temp, "{}\n", l).unwrap();
        }

        if l == "### Reekup End" {
            copy_line = true;
        }

    }

    let standard = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(".standard.reek");

    let standard = match standard {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    write!(&mut temp, "### Reekup Begin\n").unwrap();

    let standard_buffer = BufReader::new(&standard);

    for line in standard_buffer.lines() {
        let l = line.unwrap();
        write!(&mut temp, "{}\n", l).unwrap();
    }

    write!(&mut temp, "### Reekup End\n").unwrap();

    fs::copy("reekup.tmp", "config.reek").unwrap();

    fs::remove_file("reekup.tmp").unwrap();
}

fn get_reek_standards() -> Result<File, std::io::Error> {
    // Prepare the HTTP request to be sent.
    let mut req = Easy::new();
    req.get(true).unwrap();
    req.url("https://raw.githubusercontent.com/umn-asr/dotfiles/master/reek").unwrap();

    // This is called when the request is complete. It opens (or creates) config.reek and is set to
    // append at the bottom of the file.
    req.write_function(|data| {
            let mut standard =
                OpenOptions::new().write(true).create(true).open(".standard.reek").unwrap();
            standard.write_all(data).unwrap();

            Ok(data.len())
        })
        .unwrap();
    req.perform().unwrap();

    // If the request runs, do nothing. Otherwise display a generic error.
    match req.response_code() {
        Ok(_) => OpenOptions::new().write(false).read(true).create(false).open(".standard.reek"),
        Err(_) => panic!("Unable to make http request"),
    }
}
