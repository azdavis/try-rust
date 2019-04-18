pub fn run() {
  if let Some(x) = mk_ans(15) {
    println!("{}", x);
  }
}

pub fn mk_ans(x: i32) -> Option<i32> {
  if x >= 10 {
    Some(x - 1)
  } else {
    None
  }
}
