extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
  let ans = rand::thread_rng().gen_range(1, 101);
  loop {
    print!("guess the number: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("could not read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(x) => x,
      Err(e) => {
        println!("{}", e);
        continue;
      }
    };
    match guess.cmp(&ans) {
      Ordering::Less => println!("too small"),
      Ordering::Greater => println!("too big"),
      Ordering::Equal => {
        println!("nice work");
        break;
      }
    }
  }
  println!("ans = {}", ans);
}
