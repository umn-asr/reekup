extern crate curl;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;

use std::io::Write;
use std::fs::OpenOptions;

use curl::easy::Easy;
use tokio_core::reactor::Core;
use tokio_curl::Session;

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
            let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("config.reek")
                .unwrap();

            f.write_all(data).unwrap();
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
