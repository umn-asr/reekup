extern crate curl;
use self::curl::easy::Easy;

use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

pub mod options;
pub mod config_file;

use self::options::Options;

pub fn run(options: &Options) {
    let reek_standards = get_reek_standards(&options.source_url);
    match reek_standards {
        Ok(_) => update_config(&options.target_filepath),
        Err(_) => println!("Unable to make http request"),
    }
    fs::remove_file(".standard.reek").unwrap();
}

fn update_config(target_filepath: &String) {
    let mut temp_filename = target_filepath.clone();
    temp_filename.push_str(".tmp");

    let temp = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_filename);

    let mut temp = match temp {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let current = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(target_filepath);

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

    fs::copy(&temp_filename, target_filepath).unwrap();

    fs::remove_file(&temp_filename).unwrap();
}

fn get_reek_standards(source_url: &String) -> Result<File, std::io::Error> {
    // Prepare the HTTP request to be sent.
    let mut req = Easy::new();
    req.get(true).unwrap();
    req.url(source_url).unwrap();

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
