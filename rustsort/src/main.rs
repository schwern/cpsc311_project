#[macro_use]
extern crate clap;

use clap::{Arg, AppSettings};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::io;

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

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() > 1 {
            let filename = args[1].clone();
            Config {filename}
        } else {
            panic!("Not enough arguments given")
        }

    }
}

#[derive(Eq)]
pub struct KeyLinePair<'a>{
// key refers to a string slice of the line
	key: &'a str,
//	line: String
}

impl<'a> PartialOrd for KeyLinePair<'a>{
    fn partial_cmp(&self, other: &KeyLinePair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for KeyLinePair<'a>{
    fn eq(&self, other: &KeyLinePair) -> bool {
        self.key == other.key
    }
}

impl<'a> Ord for KeyLinePair<'a>{
 fn cmp(&self, other: &KeyLinePair) -> Ordering {
        self.key.cmp(&other.key)
    }
}

pub fn run(config: clap::ArgMatches) -> Result<(), io::Error>{

    let mut contents = String::new();
    let files = config.values_of_os("FILE").unwrap();

    for file in files {
        let mut f = match File::open(file){
    		Ok(f) => f,
    		Err(e) => return Err(e),
    	};

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    }


    let mut nodes = Vec::new();

    //then read line by line and collect lines in a vector
    for line in contents.lines() {
        nodes.push(line);
    };

    //call vector.sort() if Strings have total ordering
    //"A mutable slice of elements with a total ordering has a sort method"
    // https://stackoverflow.com/questions/26836488/how-to-sort-a-vector-in-rust
    nodes.sort();

    for node in nodes {
        println!("{}", node)
    };

    Ok(())
}
