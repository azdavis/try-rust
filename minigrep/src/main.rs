use std::env;
use std::error::Error;
use std::process;

extern crate minigrep;

fn fatal(err: Box<Error>) {
    eprintln!("{}", err);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = minigrep::Config::new(&args)
        .and_then(minigrep::run)
        .unwrap_or_else(fatal);
}
