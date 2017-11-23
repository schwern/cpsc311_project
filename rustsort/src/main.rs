extern crate rustsort;

use std::env;

use rustsort::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    if let Err(e) = rustsort::run(config) {
        panic!("Application error: {}", e)
    }
}
