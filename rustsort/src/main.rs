use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents) // replace this with a bufreader
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    filename: String
}
fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();
    // better to move than clone... how?
    // https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html#the-tradeoffs-of-using-clone
    
    Config {filename}
}
