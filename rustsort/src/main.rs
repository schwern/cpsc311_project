use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("In file {}", config.filename);

    if let Err(e) = run(config){
        println!("Application error: {}", e);

        process:exit(1);
    }
}

fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();
    // better to move than clone... how?
    // https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html#the-tradeoffs-of-using-clone

    Config {filename}
}
