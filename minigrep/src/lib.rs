use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents = \n{}", contents);
    Ok(())
}
