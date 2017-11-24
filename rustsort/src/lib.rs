use std::fs:: File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::io;
use std::thread;

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
	line: String
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

pub fn run(config: Config) -> Result<(), io::Error>{

    show_threading();
    //read in file here, take info from Config struct
    let mut f = match File::open(config.filename){
		Ok(file) => file,
		Err(e) => return Err(e),
	};

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut nodes = Vec::new();

    //then read line by line and collect lines in a vector
    for line in contents.lines() {
        nodes.push(line);
    }

    //call vector.sort() if Strings have total ordering
    //"A mutable slice of elements with a total ordering has a sort method"
    // https://stackoverflow.com/questions/26836488/how-to-sort-a-vector-in-rust
    nodes.sort();

    for node in nodes {
        println!("{}", node)
    }

    Ok(())
}

pub fn show_threading(){
    let handle = thread::spawn(|| {
        println!("Let's sort!")
    });

    handle.join();
}
