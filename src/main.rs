use std::io;
use std::io::Write;

fn main() {
    print!("guess the number: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("could not read line");
    println!("you guessed {}", guess);
}
