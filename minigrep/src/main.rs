use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    let mut file = File::open(config.filename).expect("no such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read");
    println!("contents = \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
