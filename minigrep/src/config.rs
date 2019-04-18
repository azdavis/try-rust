use std::env;
use std::error::Error;
use std::fmt;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
    if args.len() != 3 {
      return Err(Box::new(BadArgs(args[0].clone())));
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

#[derive(Debug)]
struct BadArgs(String);

impl Error for BadArgs {
  fn description(&self) -> &str {
    "BadArgs"
  }
}

impl fmt::Display for BadArgs {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "usage: {} <query> <filename>", self.0)
  }
}
