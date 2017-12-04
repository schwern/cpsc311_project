#[macro_use]
extern crate clap;
extern crate libc;
use libc::strcoll;

use clap::{Arg, AppSettings};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::cmp::Ordering;

fn main() {
    let matches = app_from_crate!()
        .after_help("Written for CPSC 311 at University of British Columbia")
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(Arg::with_name("KEYDEF")
            .short("k")
            .long("key")
            .takes_value(true)
            .help("sort via a key; KEYDEF gives location and type"))
            // TODO .validator()
            // https://github.com/kbknapp/clap-rs/blob/master/examples/15_custom_validator.rs
        .arg(Arg::with_name("FILE")
            .multiple(true))
        .get_matches();

    ::std::process::exit(match run(matches) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:#?}", err);
            2
        }
    });
}

fn run(config: clap::ArgMatches) -> Result<(), io::Error>{
    let mut nodes = Vec::new();
    let files = config.values_of_os("FILE").unwrap();

    for file in files {
        let f = match File::open(file){
    		Ok(f) => f,
    		Err(e) => return Err(e),
    	};

        let reader = BufReader::new(f);
        for line in reader.lines() {
            match line {
                // use MyString to be able to use it's Ord
                Ok(text) => nodes.push(MyString(text)),
                Err(e) => return Err(e),
            };
        };
    }

    nodes.sort();

    for node in nodes {
        let MyString(line) = node;
        println!("{}", line)
    };

    Ok(())
}

#[derive(Eq, PartialEq)]
struct MyString(
    String
);

impl Ord for MyString {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal // FIXME use libc::strcoll() to decide this
    }
}

impl PartialOrd for MyString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
