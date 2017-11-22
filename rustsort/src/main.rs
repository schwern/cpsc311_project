use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("In file {}", config.filename);

    run(config);
}

<<<<<<< HEAD
struct Config {
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        // better to move than clone... how?
        // https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html#the-tradeoffs-of-using-clone\
        
        Config {filename}
    }
=======

fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();
    // better to move than clone... how?
    // https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html#the-tradeoffs-of-using-clone

    Config {filename}
>>>>>>> separated lib and main
}
