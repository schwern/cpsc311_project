extern crate rustsort;

use std::env;

use rustsort::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("In file {}", config.filename);

    if let Err(e) = rustsort::run(config) {
        println!{"Application error: {}", e};
    }
}
