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
    Ok(())
}
    //read in file here, take info from Config struct
    //then read line by line and collect lines in a vector
    //call vector.sort() if Strings have total ordering
    //"A mutable slice of elements with a total ordering has a sort method"
    // https://stackoverflow.com/questions/26836488/how-to-sort-a-vector-in-rust
