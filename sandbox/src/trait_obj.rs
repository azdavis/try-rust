use std::fmt::Display;

pub fn run() {
  foo(&3);
  foo(&false);
}

// try removing 'dyn' to get a deprecation warning
fn foo(x: &dyn Display) {
  println!("{}", x);
}
