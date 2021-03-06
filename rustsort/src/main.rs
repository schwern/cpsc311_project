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
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actual_merge_empty_left() {
        let mut a : VecDeque<MyString> = vec![].into_iter().collect();
        let mut b : VecDeque<MyString> = vec!["aa", "qq", "rr", "zz"]
            .into_iter()
            .map( |x| MyString(String::from(x)) )
            .collect();
        let want : VecDeque<MyString> = vec!["aa", "qq", "rr", "zz"]
            .into_iter()
            .map( |x| MyString(String::from(x)) )
            .collect();

        let result = actual_merge(&mut a, &mut b);
        assert_eq!( result, want );
    }
    
    #[test]
    fn test_actual_merge_both_full() {
        let mut a : VecDeque<MyString> = vec!["bb", "dd", "ee"]
            .into_iter()
            .map( |x| MyString(String::from(x)) )
            .collect();
        let mut b : VecDeque<MyString> = vec!["aa", "qq", "zz"]
            .into_iter()
            .map( |x| MyString(String::from(x)) )
            .collect();
        let want : VecDeque<MyString> = vec!["aa", "bb", "dd", "ee", "qq", "zz"]
            .into_iter()
            .map( |x| MyString(String::from(x)) )
            .collect();

        let result = actual_merge(&mut a, &mut b);
        assert_eq!( result, want );
    }
}

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
        .arg(Arg::with_name("MERGE")
            .short("m")
            .long("merge")
            .help("merge already sorted files"))
        .arg(Arg::with_name("FILE")
            .multiple(true))
        .get_matches();

    // this unsafe call to libc makes sure we use the correct strcoll()
    unsafe {
        let emptystring = CString::new("").unwrap();
        libc::setlocale(libc::LC_ALL, emptystring.as_ptr());
    }

    let files = matches.values_of_os("FILE").unwrap();

    if  matches.is_present("MERGE") {
        ::std::process::exit(match merge(files) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("error: {:#?}", err);
                2
            }
        });
    } else {
        ::std::process::exit(match run(files) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("error: {:#?}", err);
                2
            }
        });
    };
}

fn run(files: clap::OsValues) -> Result<(), io::Error>{
    let mut nodes = Vec::new();

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


fn merge(files: clap::OsValues) -> Result<(), io::Error>{
    let mut nodes: VecDeque<MyString> = VecDeque::new();
    let mut pieces = VecDeque::new();

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

        nodes = actual_merge(&mut nodes, &mut pieces);
    };

    for node in nodes{
        let MyString(line) = node;
        println!("{}", line);
    }

    Ok(())

}

fn actual_merge(left: &mut VecDeque<MyString>, right: &mut VecDeque<MyString>) -> VecDeque<MyString> {
    let mut result = VecDeque::new();
    while !(left.is_empty() || right.is_empty()) {
        if left.get(0) <= right.get(0) {
            result.push_back(left.pop_front().unwrap());
        } else {
            result.push_back(right.pop_front().unwrap());
        }
    }
    
    if ! left.is_empty(){
        result.append(left);
    }
    else if !right.is_empty(){
        result.append(right);
    }
    
    result
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
