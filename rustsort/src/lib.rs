use std::error::Error;
use std::fs:: File;
use std::io::prelude::*;

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        // better to move than clone... how?
        // https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html#the-tradeoffs-of-using-clone\
        
        Config {filename}
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>>{
    //read in file here, take info from Config struct
    let mut f = File::open(config.filename).expect("file not found");

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
    
