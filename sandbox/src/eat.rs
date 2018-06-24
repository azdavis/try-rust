use std::fmt::Display;

pub fn run() {
    let s = String::from("guy");
    let _rs = &s;
    // eat(s);

    let n = 5;
    let _rn = &n;
    eat(n);
}

fn eat<T>(x: T) where T: Display {
    println!("eat {}", x);
}
