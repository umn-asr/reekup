extern crate curl;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;

use std::io::Write;
use std::fs::OpenOptions;
use std::fs;

use curl::easy::Easy;
use tokio_core::reactor::Core;
use tokio_curl::Session;

use std::io::BufReader;
use std::io::BufRead;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();
    update_reek_config();
}

fn update_reek_config() {
    // Create an event loop that we'll run on, as well as an HTTP `Session`
    // which we'll be routing all requests through.
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());


    // Prepare the HTTP request to be sent.
    let mut req = Easy::new();
    req.get(true).unwrap();
    req.url("https://raw.githubusercontent.com/umn-asr/dotfiles/master/reek").unwrap();

    // This is called when the request is complete. It opens (or creates) config.reek and is set to
    // append at the bottom of the file.
    req.write_function(|data| {
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
            write!(&mut temp, "### Reekup Begin\n").unwrap();
            temp.write_all(data).unwrap();
            write!(&mut temp, "### Reekup End\n").unwrap();

            fs::copy("reekup.tmp", "config.reek").unwrap();

            fs::remove_file("reekup.tmp").unwrap();

            Ok(data.len())
        })
        .unwrap();

    // Once we've got our session, issue an HTTP request to download the
    // rust-lang home page
    let request = session.perform(req);

    // If the request runs, do nothing. Otherwise display a generic error.
    match lp.run(request) {
        Ok(_) => (),
        Err(_) => println!("Unable to make http request"),
    }
}
