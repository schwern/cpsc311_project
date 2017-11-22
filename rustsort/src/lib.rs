use std::error::Error;
use std::fs:: File;
use std::io::prelude::*;

pub struct Config {
    filename: String
}

fn run(config: Config) -> Result<(), Box<Error>>{
    //read in file here, take info from Config struct
    //then read line by line and collect lines in a vector
    //call vector.sort() if Strings have total ordering
    //"A mutable slice of elements with a total ordering has a sort method"
    // https://stackoverflow.com/questions/26836488/how-to-sort-a-vector-in-rust



}
