extern crate rustsort;

use std::env;

use rustsort::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

<<<<<<< HEAD
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
=======
    let config = Config::new(&args);

    if let Err(e) = rustsort::run(config) {
        println!{"Application error: {}", e};
    }
>>>>>>> ffc5e5132b1d388be6be5aa6710b6def755ce83d
}
