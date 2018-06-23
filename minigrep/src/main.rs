use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <query> <file>", args[0]);
        std::process::exit(1);
    }
    let query = &args[1];
    let file = &args[2];
}
