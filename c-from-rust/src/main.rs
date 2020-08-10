use std::os::raw::c_int;

extern "C" {
  fn inc(x: c_int) -> c_int;
}

fn main() {
  let x = unsafe { inc(3) };
  println!("result: {}", x);
}
