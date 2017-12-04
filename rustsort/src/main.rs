#[macro_use]
extern crate clap;
extern crate libc;

use clap::{Arg, AppSettings};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::BufReader;
use std::cmp::Ordering;
use std::ffi::CString;
use std::collections::VecDeque;

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
        .arg(Arg::with_name("env_LANG")
            .long("collate")
            .hidden(true)
            .help("used to hold LANG from environment")
            .env("LANG"))
        .arg(Arg::with_name("FILE")
            .multiple(true))
        .get_matches();

    // this unsafe call to libc makes sure we use the correct strcoll()
    unsafe {
        let emptystring = CString::new("").unwrap();
        libc::setlocale(libc::LC_ALL, emptystring.as_ptr());
    }

    ::std::process::exit(match merge(matches) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:#?}", err);
            2
        }
    });
}

fn run(config: clap::ArgMatches) -> Result<(), io::Error>{

    let lang = config.value_of("env_LANG").unwrap_or("");
    //println!("lang is: {}", lang);

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

fn merge(config: clap::ArgMatches) -> Result<(), io::Error>{
    let mut nodes: VecDeque<MyString> = VecDeque::new();
    let mut pieces = VecDeque::new();
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
                Ok(text) => pieces.push_back(MyString(text)),
                Err(e) => return Err(e),
            };
        };

        nodes = match actual_merge(&mut nodes, &mut pieces){
           Ok(v) => v,
           Err(e) => return Err(e),
       };
    };

    for node in nodes{
        let MyString(line) = node;
        println!("{}", line);
    }

    Ok(())

}

fn actual_merge(acc: &mut VecDeque<MyString>, file2: &mut VecDeque<MyString>) -> Result<VecDeque<MyString>, io::Error>{
    let mut result = VecDeque::new();
    while !(acc.is_empty() || file2.is_empty()) {
        let head1 = match acc.pop_front() {
            Some(h) => h,
            None => return Err(Error::new(ErrorKind::UnexpectedEof, "unexpected emptiness")),
        };

        let head2 = match file2.pop_front() {
            Some(h) => h,
            None => return Err(Error::new(ErrorKind::UnexpectedEof, "unexpected emptiness")),
        };

        if head1 <= head2 {
           result.push_back(head1);
           file2.push_front(head2);
       } else {
           result.push_back(head2);
           acc.push_front(head1);
    };

  }
    if ! acc.is_empty(){
      result.append(acc);
    } else if !file2.is_empty(){
      result.append(file2);
    }

  Ok(result)
 }
