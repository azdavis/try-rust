extern crate minigrep;

use std::env;
use std::error::Error;
use std::process;

fn fatal(err: Box<dyn Error>) {
    eprintln!("{}", err);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = minigrep::config::Config::new(&args)
        .and_then(minigrep::run)
        .unwrap_or_else(fatal);
}
