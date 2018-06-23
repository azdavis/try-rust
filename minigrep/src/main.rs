use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <query> <file>", args[0]);
        std::process::exit(1);
    }
    let query = &args[1];
    let file = &args[2];
    let mut file = File::open(file).expect("no such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read");
    println!("contents = \n{}", contents);
}
