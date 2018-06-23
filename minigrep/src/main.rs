fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    if args.len() != 3 {
        eprintln!("usage: {} <regex> <file>", args[0]);
        std::process::exit(1);
    }
    let regex = &args[1];
    let file = &args[2];
    println!("regex = {}, file = {}", regex, file);
}
