#[macro_use]
extern crate clap;
extern crate rustsort;
use clap::App;

use std::env;
use std::io::{self, Write};


fn main() {
    let matches = app_from_crate!()
        .after_help("Written for CPSC 311 at University of British Columbia")
        .get_matches();

    let args: Vec<String> = env::args().collect();

    let config = rustsort::Config::new(&args);

    // let's try it this way:
    // https://doc.rust-lang.org/std/process/fn.exit.html
    ::std::process::exit(match rustsort::run(config) {
        Ok(_) => 0,
        Err(err) => {
            writeln!(io::stderr(), "error: {:?}", err).unwrap();
            2
        }
    });
}
