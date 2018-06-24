pub fn run() {
    let s = String::from("guy");
    let _rs = &s;
    // eat(s);

    let n = 5;
    let _rn = &n;
    eat(n);
}

fn eat<T>(_x: T) {
    println!("eat");
}
