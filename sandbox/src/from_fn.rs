fn fib() -> impl Iterator<Item = u64> {
  let mut x = 0;
  let mut y = 1;
  std::iter::from_fn(move || {
    let z = x;
    x = y;
    y += z;
    Some(z)
  })
}

pub fn run() {
  for x in fib().take(10) {
    println!("{}", x);
  }
}
