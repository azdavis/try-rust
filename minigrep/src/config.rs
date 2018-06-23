use std::error::Error;
use std::fmt;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, Box<Error>> {
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
