pub fn run() {
  let xs = &[2, 4, 6];
  // panics if end > xs.len()
  let end = xs.len();
  if xs[2..end] == [6] {
    println!("ok");
  }
}
