#[macro_use]
extern crate clap;

use clap::{Arg, AppSettings};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;

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

pub fn run(config: clap::ArgMatches) -> Result<(), io::Error>{
    let mut nodes = Vec::new();
    let files = config.values_of_os("FILE").unwrap();

    for file in files {
        let f = match File::open(file){
    		Ok(f) => f,
    		Err(e) => return Err(e),
    	};

        let reader = BufReader::new(f);
        for line in reader.lines() {
            nodes.push(line.unwrap());
        };
    }

    nodes.sort();

    for node in nodes {
        println!("{}", node)
    };

    Ok(())
}
