extern crate rand;

use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    let ans = rand::thread_rng().gen_range(1, 101);
    print!("guess the number: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("could not read line");
    println!("you guessed {}", guess);
    println!("the true answer is {}", ans);
}
