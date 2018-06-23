use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn fatal(err: Box<Error>) {
    eprintln!("{}", err);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = Config::new(&args)
        .and_then(run)
        .unwrap_or_else(fatal);
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents = \n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, Box<Error>> {
        if args.len() != 3 {
            return Err(Box::new(BadArgs));
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

#[derive(Debug)]
struct BadArgs;

impl Error for BadArgs {
    fn description(&self) -> &str {
        "bad args"
    }
}

impl fmt::Display for BadArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
