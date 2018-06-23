use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <regex> <file>", args[0]);
        std::process::exit(1);
    }
    let regex = &args[1];
    let file = &args[2];
}
