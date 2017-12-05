#[macro_use]
extern crate clap;
extern crate libc;

use clap::{Arg, AppSettings};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::cmp::Ordering;
use std::ffi::CString;

fn main() {
    let matches = app_from_crate!()
        .after_help("Written for CPSC 311 at University of British Columbia")
        .setting(AppSettings::UnifiedHelpMessage)
/*
        .arg(Arg::with_name("KEYDEF")
            .short("k")
            .long("key")
            .takes_value(true)
            .help("sort via a key; KEYDEF gives location and type"))
            // TODO .validator()
            // https://github.com/kbknapp/clap-rs/blob/master/examples/15_custom_validator.rs
*/
        .arg(Arg::with_name("MERGE")
            .short("m")
            .long("merge")
            .help("merge already sorted files; do not sort"))
        .arg(Arg::with_name("FILE")
            .multiple(true))
        .get_matches();

    unsafe { // this unsafe call to libc makes sure we use the correct strcoll()
        let emptystring = CString::new("").unwrap();
        libc::setlocale(libc::LC_ALL, emptystring.as_ptr());
    }

    ::std::process::exit(match run(matches) {
        Ok(_) => 0,
/*        // POSIX requires that sort return 1 IFF invoked with -c or -C and the
        // input is not properly sorted.
        Err(err) => if err == {
            eprintln!("error: {:#?}", err);
            1
        }*/
        Err(err) => {
            eprintln!("error: {:#?}", err);
            2
        }
    });
}

fn run(config: clap::ArgMatches) -> Result<(), io::Error>{
    let files = config.values_of_os("FILE").unwrap();
    let mut texts: Vec<Vec<MyString>> = Vec::new();

    for file in files {
        let f = File::open(file)?;

        let mut text = Vec::new();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            match line {
                // use MyString to be able to use it's Ord
                Ok(l) => text.push(MyString(l)),
                Err(e) => return Err(e),
            };
        };
        texts.push(text);
    }

    if !config.is_present("MERGE") {
        for text in &mut texts {
            text.sort();
        }
    }

    // if there's more than one text, merge them
    while texts.len() > 1 {
        // FIXME stacks all text into one Vec, whoops
        let text1 = texts.pop().unwrap();
        let text2 = texts.pop().unwrap();
        let mut t1 = text1.into_iter();
        let mut t2 = text2.into_iter();
        let mut text3: Vec<MyString> = Vec::new();
        loop {
            match(t1.next(), t2.next()) {
                (Some(s1), Some(s2)) => {
                    if s1 > s2  {
                        text3.push(s2);
                        t2.next();
                    } else {
                        text3.push(s1);
                        t1.next();
                    }
                },
                (None, Some(s2)) => {
                    text3.push(s2);
                    t2.next();
                },
                (Some(s1), None) =>  {
                    text3.push(s1);
                    t1.next();
                },
                (None, None) => break,
            }
        }
        texts.push(text3);
    }

    for text in texts {
        for node in text {
            let MyString(line) = node;
            println!("{}", line);
        }
    };

    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
struct MyString(
    String
);

impl Ord for MyString {
    fn cmp(&self, other: &Self) -> Ordering {
        let &MyString(ref s1_rusty) = self;
        let &MyString(ref s2_rusty) = other;
        let s1 = CString::new(s1_rusty.as_bytes()).unwrap();
        let s2 = CString::new(s2_rusty.as_bytes()).unwrap();
// the above gyrations are for turning our Rust strings into null-terminated
// C strings that can safely interact with the libc function below
        let result = unsafe {
            libc::strcoll(s1.as_ptr(), s2.as_ptr())
        };
        match result {
            _ if result > 0 => Ordering::Greater,
            _ if result < 0 => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for MyString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
